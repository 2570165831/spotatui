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

  /// Start one Music library track. The browse screens will add the playlist
  /// context later; the first version plays the track from the library.
  pub(crate) fn play_apple_music_track(&mut self, uri: String) {
    self.dispatch(IoEvent::StartPlayback(None, Some(vec![uri]), None));
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
}
