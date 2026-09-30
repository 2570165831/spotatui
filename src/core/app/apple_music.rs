use super::*;

impl App {
  #[cfg(all(feature = "macos-media", target_os = "macos"))]
  pub(crate) fn install_macos_media_manager(
    &mut self,
    manager: Option<Arc<crate::infra::macos_media::MacMediaManager>>,
  ) {
    self.macos_media_manager = manager;
  }

  pub(crate) fn apple_music_state(&self) -> &crate::infra::apple_music::RemoteState {
    &self.apple_music
  }

  pub(crate) fn apple_music_owns_playback(&self) -> bool {
    self.apple_music.claimed
  }

  pub(crate) fn apple_music_position_ms(&self) -> u32 {
    let Some(snapshot) = &self.apple_music.snapshot else {
      return 0;
    };
    let elapsed = self
      .apple_music
      .observed_at
      .map_or(0, |at| at.elapsed().as_millis());
    let extra = if snapshot.playing && elapsed < 10_000 {
      elapsed
    } else {
      0
    };
    (u128::from(snapshot.position_ms) + extra)
      .min(snapshot.track.as_ref().map_or(0, |t| t.duration_ms) as u128) as u32
  }

  pub(crate) fn apple_music_is_playing(&self) -> bool {
    self.apple_music.desired_playing
  }

  pub(crate) fn apple_music_volume(&self) -> u8 {
    self
      .apple_music
      .snapshot
      .as_ref()
      .map_or(self.runtime_state.volume_percent, |s| s.volume)
  }

  pub(crate) fn toggle_apple_music(&mut self) {
    if self.apple_music.switching {
      self.set_status_message("Waiting for Music to pause before switching source", 4);
      return;
    }
    let playing = self.apple_music_is_playing();
    self.apple_music.desired_playing = !playing;
    if let Some(snapshot) = &mut self.apple_music.snapshot {
      snapshot.playing = !playing;
    }
    self.dispatch(if playing {
      IoEvent::PausePlayback
    } else {
      IoEvent::StartPlayback(None, None, None)
    });
  }

  pub(crate) fn set_apple_music_volume(&mut self, value: u8) {
    let value = value.min(100);
    if let Some(snapshot) = &mut self.apple_music.snapshot {
      snapshot.volume = value;
    }
    self.dispatch(IoEvent::ChangeVolume(value));
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn claim_apple_music(&mut self) -> u64 {
    // Claim first: a media key during launch/permission handling must never
    // resume the player we are handing off from.
    self.apple_music.claimed = true;
    self.apple_music.desired_playing = true;
    #[cfg(all(feature = "macos-media", target_os = "macos"))]
    if let Some(manager) = &self.macos_media_manager {
      manager.set_remote_owned(true);
    }
    self.apple_music.switching = false;
    self.apple_music.generation = self.apple_music.generation.wrapping_add(1);
    self.apple_music.snapshot = None;
    self.apple_music.observed_at = None;
    self.cancel_volume_change();
    self.pending_api_seek = None;
    self.pending_source_seek = None;
    self.seek_ms = None;
    #[cfg(feature = "streaming")]
    {
      self.pending_native_seek = None;
      self.release_native_for_decoded();
    }
    #[cfg(feature = "audio-decode")]
    {
      #[allow(unused_mut)]
      let mut players = self.take_decoded_sessions_except(Source::AppleMusic);
      #[cfg(feature = "audio-decode-queue")]
      players.extend(self.take_queue_now_decoded_player());
      for player in players {
        player.pause();
        player.stop_detached();
      }
    }
    #[cfg(feature = "queue")]
    {
      self.queue_now = None;
      self.queue_suspended = None;
      self.queue_slot_desired_playing = false;
    }
    self.release_decoded_sink_claim();
    self.apple_music.generation
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn begin_apple_music_handoff(&mut self) -> u64 {
    self.apple_music.generation = self.apple_music.generation.wrapping_add(1);
    self.apple_music.switching = true;
    self.apple_music.generation
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn apple_music_handoff_is_current(&self, generation: u64) -> bool {
    self.apple_music.claimed
      && self.apple_music.switching
      && self.apple_music.generation == generation
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn acknowledge_apple_music_pause(
    &mut self,
    generation: u64,
    snapshot: crate::infra::apple_music::Snapshot,
  ) {
    if self.apple_music.claimed
      && self.apple_music.switching
      && self.apple_music.generation == generation
    {
      self.apple_music.desired_playing = false;
      self.apple_music.snapshot = Some(snapshot);
      self.apple_music.observed_at = Some(Instant::now());
    }
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn finish_apple_music_handoff(&mut self, generation: u64) -> bool {
    if !self.apple_music_handoff_is_current(generation) {
      return false;
    }
    self.apple_music.claimed = false;
    #[cfg(all(feature = "macos-media", target_os = "macos"))]
    if let Some(manager) = &self.macos_media_manager {
      manager.set_remote_owned(false);
    }
    self.apple_music.switching = false;
    self.apple_music.snapshot = None;
    self.apple_music.observed_at = None;
    true
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn accept_apple_music_snapshot(
    &mut self,
    generation: u64,
    snapshot: crate::infra::apple_music::Snapshot,
  ) {
    if self.apple_music.claimed
      && self.apple_music.generation == generation
      && !self.apple_music.switching
    {
      self.song_progress_ms = snapshot.position_ms as u128;
      self.apple_music.desired_playing = snapshot.playing;
      self.apple_music.snapshot = Some(snapshot);
      self.apple_music.observed_at = Some(Instant::now());
      self.note_display_changes();
    }
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn apple_music_failed(&mut self, generation: u64, error: anyhow::Error) {
    if self.apple_music.generation != generation {
      return;
    }
    // Retain the claim on failure: a timed-out Apple Event may have executed.
    // Only an acknowledged pause can release it to a different player.
    self.apple_music.switching = false;
    self.apple_music.snapshot = None;
    // Keep the last intent: a failed Play may already be audible, so the next
    // toggle must request Pause rather than accidentally issuing another Play.
    self.handle_error(error);
  }

  pub(crate) fn apple_music_playlists(&self) -> &[PlaylistInfo] {
    &self.apple_music.playlists
  }

  pub(crate) fn cancel_apple_music_browse(&mut self) {
    self.apple_music.browse_generation = self.apple_music.browse_generation.wrapping_add(1);
    self.apple_music.browse = None;
  }

  /// Load a Music list page by page. The sidebar opens with a synthetic
  /// "All songs" row so the library is reachable before any playlist arrives.
  pub(crate) fn browse_apple_music(&mut self, request: crate::infra::apple_music::Browse) {
    if !cfg!(all(feature = "apple-music", target_os = "macos")) {
      self.set_status_message("Apple Music requires macOS and the apple-music feature", 5);
      return;
    }
    self.cancel_apple_music_browse();
    self.apple_music.browse = Some(request.clone());
    self.apple_music.tracks.clear();
    if request == crate::infra::apple_music::Browse::Playlists {
      self.apple_music.playlists = vec![PlaylistInfo {
        uri: crate::infra::apple_music::LIBRARY_URI.into(),
        name: "All songs".into(),
        owner: "Music".into(),
        track_count: 0,
        id: None,
        owner_id: None,
        collaborative: false,
        public: None,
        image_url: None,
      }];
      self.display_revisions.bump(DisplayDomain::Library);
    }
    self.dispatch_without_spinner(IoEvent::AppleMusicPage {
      request,
      offset: 0,
      generation: self.apple_music.browse_generation,
    });
  }

  /// A page is only accepted for the list still on screen: same source, same
  /// request, same generation, and for a playlist the Music track table.
  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn apple_music_browse_is_current(
    &self,
    request: &crate::infra::apple_music::Browse,
    generation: u64,
  ) -> bool {
    self.active_source == Source::AppleMusic
      && self.apple_music.browse_generation == generation
      && self.apple_music.browse.as_ref() == Some(request)
      && (!matches!(request, crate::infra::apple_music::Browse::Tracks(_))
        || self.track_table.context == Some(TrackTableContext::AppleMusicPlaylist))
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn append_apple_music_tracks(&mut self, tracks: Vec<TrackInfo>, search: bool) {
    self.apple_music.tracks.extend(tracks);
    let tracks = self.apple_music.tracks.clone();
    if search {
      self.set_search_results(SearchResult {
        tracks: Some(Paged {
          total: tracks.len() as u32,
          items: tracks,
          ..Default::default()
        }),
        ..Default::default()
      });
      self.view.search_hovered_block = SearchResultBlock::SongSearch;
    } else {
      self.replace_track_table_tracks(tracks);
    }
  }

  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub(crate) fn append_apple_music_playlists(&mut self, playlists: Vec<PlaylistInfo>) {
    self.apple_music.playlists.extend(playlists);
    self.display_revisions.bump(DisplayDomain::Library);
  }

  /// Start one Music track, inside the playlist on screen when it came from
  /// one, so Music's own next/previous stay within that playlist.
  pub(crate) fn play_apple_music_track(&mut self, uri: String) {
    if let Some(crate::infra::apple_music::Browse::Tracks(context)) =
      self.apple_music.browse.as_ref().filter(|_| {
        self.track_table.context == Some(TrackTableContext::AppleMusicPlaylist)
          && self
            .track_table
            .tracks
            .iter()
            .any(|t| t.uri.as_deref() == Some(uri.as_str()))
      })
    {
      self.start_playback_track_in_context(context.clone(), uri);
    } else {
      self.dispatch(IoEvent::StartPlayback(None, Some(vec![uri]), None));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn apple_music_owner_routes_without_spotify_or_a_decoded_player() {
    let (tx, rx) = channel();
    let mut app = App::new(tx, UserConfig::new(), None);
    let generation = app.claim_apple_music();
    assert_eq!(app.playback_owner(), PlaybackOwner::AppleMusic);
    assert!(!app.active_decoded_source());
    assert!(!app.native_should_drive());
    app.toggle_playback();
    assert!(matches!(rx.try_recv(), Ok(IoEvent::PausePlayback)));
    app.toggle_playback();
    assert!(matches!(
      rx.try_recv(),
      Ok(IoEvent::StartPlayback(None, None, None))
    ));
    app.next_track();
    assert!(matches!(rx.try_recv(), Ok(IoEvent::NextTrack)));
    app.force_previous_track();
    assert!(matches!(rx.try_recv(), Ok(IoEvent::ForcePreviousTrack)));
    app.seek_to(5000);
    assert!(matches!(rx.try_recv(), Ok(IoEvent::Seek(5000))));
    app.set_volume_percent(23);
    assert!(matches!(rx.try_recv(), Ok(IoEvent::ChangeVolume(23))));
    let newer = app.begin_apple_music_handoff();
    assert!(!app.finish_apple_music_handoff(generation));
    assert!(app.finish_apple_music_handoff(newer));
    assert!(!app.apple_music_owns_playback());
  }

  #[test]
  fn apple_music_claim_survives_failure_and_a_source_switch() {
    let mut app = App::default();
    let generation = app.claim_apple_music();
    app.apple_music_failed(generation, anyhow!("timeout"));
    assert_eq!(app.playback_owner(), PlaybackOwner::AppleMusic);
    assert!(app.apple_music_is_playing());
    // Browse scope has no bearing on the actual playback owner.
    app.active_source = Source::Local;
    assert_eq!(app.playback_owner(), PlaybackOwner::AppleMusic);
    assert!(!PlaybackOwner::AppleMusic.owns_local_sink());
  }

  #[test]
  fn apple_music_stale_handoff_and_snapshot_cannot_replace_a_new_start() {
    let mut app = App::default();
    let old = app.claim_apple_music();
    let handoff = app.begin_apple_music_handoff();
    let current = app.claim_apple_music();
    let snapshot = crate::infra::apple_music::parse_snapshot(r#"{"running":true,"playing":true,"track":{"id":"0123456789ABCDEF","name":"Music track","artist":"Artist","album":"Album","duration":100},"position":1,"volume":23}"#).unwrap();
    app.accept_apple_music_snapshot(old, snapshot.clone());
    assert!(app.apple_music_state().snapshot.is_none());
    assert!(!app.finish_apple_music_handoff(handoff));
    app.accept_apple_music_snapshot(current, snapshot);
    let metadata = crate::infra::media_metadata::current_playback_snapshot(&app).unwrap();
    assert_eq!(
      metadata.source,
      crate::infra::media_metadata::PlaybackSource::AppleMusic
    );
    assert_eq!(metadata.metadata.album, "Album");
    assert_eq!(metadata.metadata.title, "Music track");
    let device = crate::core::plugin_api::playback_state(&app)
      .unwrap()
      .device
      .unwrap();
    assert_eq!(device.name, "Music.app");
    assert_eq!(device.volume_percent, Some(23));
  }

  #[test]
  fn apple_music_queue_requests_never_reach_spotify_or_the_native_queue() {
    let (tx, rx) = channel();
    let mut app = App::new(tx, UserConfig::new(), None);
    app.add_to_spotify_queue("applemusic:0123456789ABCDEF".into());
    assert!(rx.try_recv().is_err());
    app.claim_apple_music();
    app.add_to_spotify_queue("spotify:track:other".into());
    assert!(rx.try_recv().is_err());
    assert!(app.native_queue.is_empty());
  }

  #[test]
  fn apple_music_transport_keys_go_to_music_and_not_to_spotify() {
    let (tx, rx) = channel();
    let mut app = App::new(tx, UserConfig::new(), None);
    app.claim_apple_music();
    // No snapshot yet, so the claim's desired state reads as playing.
    app.toggle_playback();
    app.next_track();
    app.previous_track();
    app.transfer_playback_to_device("device".into(), false);
    let sent: Vec<IoEvent> = rx.try_iter().collect();
    assert!(matches!(
      sent.as_slice(),
      [
        IoEvent::PausePlayback,
        IoEvent::NextTrack,
        IoEvent::PreviousTrack
      ]
    ));
    assert!(!app.apple_music_is_playing());
    app.toggle_playback();
    assert!(matches!(
      rx.try_recv(),
      Ok(IoEvent::StartPlayback(None, None, None))
    ));
  }

  #[test]
  fn apple_music_late_pages_are_rejected_once_the_list_moved_on() {
    use crate::infra::apple_music::{Browse, LIBRARY_URI};
    let mut app = App::default();
    app.active_source = Source::AppleMusic;
    let request = Browse::Tracks(LIBRARY_URI.into());
    app.apple_music.browse = Some(request.clone());
    app.track_table.context = Some(TrackTableContext::AppleMusicPlaylist);
    assert!(app.apple_music_browse_is_current(&request, 0));
    // A newer browse bumps the generation.
    app.cancel_apple_music_browse();
    assert!(!app.apple_music_browse_is_current(&request, 0));
    // Another table replaced the Music one.
    app.apple_music.browse = Some(request.clone());
    app.set_track_table(Vec::new(), TrackTableContext::SavedTracks);
    assert!(!app.apple_music_browse_is_current(&request, 2));
    // The user switched sources.
    app.apple_music.browse = Some(request.clone());
    app.track_table.context = Some(TrackTableContext::AppleMusicPlaylist);
    app.set_active_source(Source::Local);
    assert!(!app.apple_music_browse_is_current(&request, 3));
  }

  #[cfg(all(feature = "apple-music", target_os = "macos"))]
  #[test]
  fn apple_music_sidebar_starts_with_all_songs_and_a_track_plays_in_its_playlist() {
    use crate::infra::apple_music::{Browse, LIBRARY_URI};
    let (tx, rx) = channel();
    let mut app = App::new(tx, UserConfig::new(), None);
    app.set_active_source(Source::AppleMusic);
    app.load_source_sidebar(Source::AppleMusic);
    assert_eq!(app.apple_music_playlists()[0].uri, LIBRARY_URI);
    assert!(matches!(
      rx.try_recv(),
      Ok(IoEvent::AppleMusicPage {
        request: Browse::Playlists,
        offset: 0,
        ..
      })
    ));

    let playlist = "applemusic:playlist:0123456789ABCDEF".to_string();
    let track = "applemusic:FEDCBA9876543210".to_string();
    app.open_source_playlist_tracks(playlist.clone());
    assert!(matches!(
      rx.try_recv(),
      Ok(IoEvent::AppleMusicPage { request: Browse::Tracks(ref uri), .. }) if *uri == playlist
    ));
    app.append_apple_music_tracks(
      vec![crate::core::app::test_support::queue_track(
        Some(&track),
        "Song",
      )],
      false,
    );
    app.start_playback_uris(vec![track.clone()], Some(0));
    assert!(matches!(
      rx.try_recv(),
      Ok(IoEvent::StartPlayback(Some(ref context), Some(ref uris), Some(0)))
        if *context == playlist && uris[..] == [track]
    ));
  }
}
