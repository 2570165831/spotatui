//! User-visible infrastructure status messages. Raw diagnostics remain untranslated.

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub struct InfraStatusStrings {
  pub sync_spotify_not_connected: &'static str,
  pub sync_qobuz_not_logged_in: &'static str,
  pub sync_subsonic_not_configured: &'static str,
  pub sync_source_not_compiled: &'static str,
  pub sync_source_unsupported: &'static str,
  pub sync_source_unreachable: &'static str,
  pub search_query_unavailable: &'static str,
  pub made_for_you_unavailable: &'static str,
  pub made_for_you_missing: &'static str,
  pub history_track_hit: &'static str,
  pub history_play: &'static str,
  pub local_replay_failed: &'static str,
  pub local_no_tracks: &'static str,
  pub local_no_folder: &'static str,
  pub local_uri_invalid: &'static str,
  pub local_play_failed: &'static str,
  pub local_task_failed: &'static str,
  pub local_audio_unavailable: &'static str,
  pub audio_init_failed: &'static str,
  pub local_scan_failed: &'static str,
  pub local_scan_task_failed: &'static str,
  pub local_read_failed: &'static str,
  pub local_tracks_scan_failed: &'static str,
  pub local_exhausted: &'static str,
  pub subsonic_audio_unavailable: &'static str,
  pub youtube_audio_unavailable: &'static str,
  pub radio_audio_unavailable: &'static str,
  pub qobuz_audio_unavailable: &'static str,
  pub subsonic_no_tracks: &'static str,
  pub subsonic_uri_invalid: &'static str,
  pub subsonic_replay_failed: &'static str,
  pub subsonic_playlists_failed: &'static str,
  pub subsonic_playlist_failed: &'static str,
  pub subsonic_search_failed: &'static str,
  pub subsonic_download_failed: &'static str,
  pub subsonic_play_failed: &'static str,
  pub subsonic_task_failed: &'static str,
  pub subsonic_no_server: &'static str,
  pub radio_no_skip: &'static str,
  pub radio_empty: &'static str,
  pub radio_uri_invalid: &'static str,
  pub radio_probe_timeout: &'static str,
  pub radio_search_failed: &'static str,
  pub radio_open_failed: &'static str,
  pub radio_play_failed: &'static str,
  pub radio_task_failed: &'static str,
  pub radio_playing: &'static str,
  pub youtube_playlist_missing: &'static str,
  pub youtube_metadata_missing: &'static str,
  pub youtube_no_tracks: &'static str,
  pub youtube_no_live: &'static str,
  pub youtube_uri_invalid: &'static str,
  pub youtube_replay_failed: &'static str,
  pub youtube_search_failed: &'static str,
  pub youtube_playlists_failed: &'static str,
  pub youtube_create_failed: &'static str,
  pub youtube_delete_failed: &'static str,
  pub youtube_add_failed: &'static str,
  pub youtube_remove_failed: &'static str,
  pub youtube_download_failed: &'static str,
  pub youtube_task_failed: &'static str,
  pub youtube_created: &'static str,
  pub youtube_deleted: &'static str,
  pub youtube_removed: &'static str,
  pub fetching: &'static str,
  pub youtube_added: &'static str,
  pub youtube_already_added: &'static str,
  pub youtube_decode_failed: &'static str,
  pub qobuz_login_expired: &'static str,
  pub qobuz_login_busy: &'static str,
  pub qobuz_browser_login: &'static str,
  pub qobuz_logged_in: &'static str,
  pub qobuz_no_tracks: &'static str,
  pub qobuz_uri_invalid: &'static str,
  pub qobuz_replay_failed: &'static str,
  pub qobuz_login_url: &'static str,
  pub qobuz_login_failed: &'static str,
  pub qobuz_step_constants: &'static str,
  pub qobuz_step_login: &'static str,
  pub qobuz_step_library: &'static str,
  pub qobuz_step_tracks: &'static str,
  pub qobuz_step_search: &'static str,
  pub qobuz_step_stream: &'static str,
  pub qobuz_operation_failed: &'static str,
  pub music_busy_playback: &'static str,
  pub music_wait_pause: &'static str,
  pub music_not_running: &'static str,
  pub music_shuffle_repeat: &'static str,
  pub music_switch_spotify: &'static str,
  pub music_busy: &'static str,
  pub music_queue_unsupported: &'static str,
  pub music_loading: &'static str,
  pub music_loaded: &'static str,
  pub music_status_unavailable: &'static str,
  pub queue_repeat_unsupported: &'static str,
  pub queue_shuffle_unsupported: &'static str,
  pub queue_no_uri: &'static str,
  pub queue_finished: &'static str,
  pub audio_disconnected: &'static str,
  pub radio_not_resumed: &'static str,
  pub source_playback_unavailable: &'static str,
  pub queue_skip_disconnected: &'static str,
  pub queue_play_failed: &'static str,
  pub queue_restore_failed: &'static str,
  pub queue_playing: &'static str,
  pub queue_audio_unavailable: &'static str,
  pub spotify_not_connected: &'static str,
  pub playlist_id_invalid: &'static str,
  pub music_requires_macos: &'static str,
  pub spotify_not_compiled: &'static str,
  pub spotify_connected: &'static str,
  pub party_disconnected: &'static str,
  pub party_ended: &'static str,
  pub spotify_login_timeout: &'static str,
  pub spotify_login_failed: &'static str,
  pub spotify_rate_wait: &'static str,
  pub spotify_pair: &'static str,
  pub recap_created: &'static str,
  pub history_failed: &'static str,
  pub recap_failed: &'static str,
  pub party_start_failed: &'static str,
  pub party_join_failed: &'static str,
  pub party_error: &'static str,
  pub party_joined: &'static str,
  pub party_left: &'static str,
  pub native_reconnecting: &'static str,
  pub native_settling: &'static str,
  pub shuffle_not_applied: &'static str,
  pub spotify_rate_retry: &'static str,
  pub spotify_poll_retry: &'static str,
  pub spotify_server_retry: &'static str,
  pub spotify_poll_rejected: &'static str,
  pub native_resuming: &'static str,
  pub native_reconnect_resume: &'static str,
  pub native_connection_restored: &'static str,
  pub native_no_state: &'static str,
  pub native_restore_retry: &'static str,
  pub other_source_playing: &'static str,
  pub spotify_playing_elsewhere: &'static str,
  pub queue_added: &'static str,
  pub queue_load_failed: &'static str,
  pub native_nothing_to_resume: &'static str,
  pub native_start_failed: &'static str,
  pub native_shuffle_failed: &'static str,
  pub native_resume_failed: &'static str,
  pub shuffle_context_failed: &'static str,
  pub shuffle_capped: &'static str,
  pub spotify_profile_retry: &'static str,
  pub friend_added: &'static str,
  pub friend_unfollowed: &'static str,
  pub friend_added_code: &'static str,
  pub friend_add_failed: &'static str,
  pub friend_unfollow_failed: &'static str,
  pub playlist_external_unavailable: &'static str,
  pub liked_check_failed: &'static str,
  pub playlist_search_scope: &'static str,
  pub playlist_search_matches: &'static str,
  pub playlist_refresh_partial: &'static str,
  pub playlist_added: &'static str,
  pub playlist_removed: &'static str,
  pub playlist_created_no_id: &'static str,
  pub playlist_added_named: &'static str,
  pub playlist_created: &'static str,
  pub native_recovered: &'static str,
  pub native_lost_reconnecting: &'static str,
  pub native_reconnected: &'static str,
  pub native_restored: &'static str,
  pub native_restored_paused: &'static str,
  pub native_skip_unavailable: &'static str,
  pub native_many_unavailable: &'static str,
  pub native_recovery_dropped: &'static str,
  pub native_recovery_failed: &'static str,
  pub native_lost_recovering: &'static str,
  pub native_stalled: &'static str,
  pub native_stopped: &'static str,
  pub native_moved: &'static str,
  pub native_disconnected: &'static str,
  pub native_stopped_recovering: &'static str,
  pub sync_busy: &'static str,
  pub sync_no_links: &'static str,
  pub sync_dry_run: &'static str,
  pub sync_removed: &'static str,
  pub sync_failed: &'static str,
  pub sync_link_failed: &'static str,
  pub sync_remove_failed: &'static str,
  pub sync_summary: &'static str,
  pub sync_using_playlist: &'static str,
  pub sync_progress: &'static str,
  pub dj_indexed: &'static str,
  pub dj_index_capped: &'static str,
  pub dj_queued: &'static str,
  pub dj_not_found: &'static str,
  pub dj_owned: &'static str,
  pub dj_duplicate: &'static str,
  pub dj_status: &'static str,
  pub dj_library_failed: &'static str,
  pub dj_topped_up: &'static str,
  pub dj_nothing_queued: &'static str,
  pub dj_error: &'static str,
  pub dj_steps_exhausted: &'static str,
  pub history_plays: &'static str,
  pub history_track_hits: &'static str,
  pub history_minutes: &'static str,
  pub history_hours_minutes: &'static str,
  pub setup_no_api_key: &'static str,
  pub setup_keep_command: &'static str,
  pub setup_anthropic: &'static str,
  pub setup_openai: &'static str,
  pub setup_own_command: &'static str,
  pub setup_not_installed: &'static str,
  pub setup_anthropic_billing: &'static str,
  pub setup_needs_key: &'static str,
  pub setup_openai_endpoint: &'static str,
  pub setup_claude_plan: &'static str,
  pub setup_codex_plan: &'static str,
  pub setup_agy_plan: &'static str,
  pub setup_copilot_plan: &'static str,
  pub setup_opencode_plan: &'static str,
  pub setup_gemini_legacy: &'static str,
  pub setup_agent_subscription: &'static str,
  pub setup_keep_as_is: &'static str,
  pub setup_own_model_note: &'static str,
  pub setup_current: &'static str,
  pub setup_cli_default: &'static str,
  pub setup_no_model_flag: &'static str,
  pub setup_builtin_default: &'static str,
  pub setup_custom: &'static str,
  pub setup_type_provider_model: &'static str,
  pub setup_type_model: &'static str,
  pub setup_haiku_note: &'static str,
  pub setup_sonnet_note: &'static str,
  pub setup_opus_note: &'static str,
  pub setup_fable_note: &'static str,
  pub setup_cheapest: &'static str,
  pub setup_heaviest_gemini: &'static str,
  pub setup_heaviest_all: &'static str,
  pub setup_price_haiku: &'static str,
  pub setup_price_sonnet: &'static str,
  pub setup_price_opus: &'static str,
  pub setup_price_fable: &'static str,
  pub history_week: &'static str,
  pub history_month: &'static str,
  pub history_all: &'static str,
  pub history_this_month: &'static str,
  pub history_this_year: &'static str,
  pub mcp_status: &'static str,
  pub network_error: &'static str,
  pub network_rate_error: &'static str,
  pub monthly_recap_ready: &'static str,
  pub queue_local_source: &'static str,
  pub unnamed_folder: &'static str,
  pub local_root_folder: &'static str,
  pub local_unknown_track: &'static str,
  pub cast_no_answer: &'static str,
  pub cast_no_tls: &'static str,
  pub cast_message_size: &'static str,
  pub cast_malformed: &'static str,
  pub cast_no_reply: &'static str,
  pub cast_receiver_failed: &'static str,
  pub cast_client_id_missing: &'static str,
  pub cast_login_refused: &'static str,
  pub cast_access_token_missing: &'static str,
  pub speaker_token_failed: &'static str,
  pub speaker_token_unsupported: &'static str,
  pub speaker_client_id_missing: &'static str,
  pub speaker_login_refused: &'static str,
  pub speaker_not_joined: &'static str,
  pub playlist_changed: &'static str,
  pub spotify_api_error: &'static str,
}

// Each helper's callers sit behind an optional source or frontend, so only the
// build with all of them (CI's all-sources leg) checks that none is dead.
#[cfg_attr(
  not(all(
    feature = "tui",
    feature = "streaming",
    feature = "discord-rpc",
    feature = "scripting",
    feature = "mcp-server",
    feature = "ai-dj",
    feature = "local-files",
    feature = "subsonic",
    feature = "internet-radio",
    feature = "youtube",
    feature = "qobuz"
  )),
  allow(dead_code)
)]
impl InfraStatusStrings {
  pub fn sync_source_not_compiled(&self, source: &str, feature: &str) -> String {
    super::format_template(
      self.sync_source_not_compiled,
      &[("source", source), ("feature", feature)],
    )
  }
  pub fn sync_source_unsupported(&self, source: &str) -> String {
    super::format_template(self.sync_source_unsupported, &[("source", source)])
  }
  pub fn sync_source_unreachable(&self, source: &str, error: &str) -> String {
    super::format_template(
      self.sync_source_unreachable,
      &[("source", source), ("error", error)],
    )
  }
  pub fn search_query_unavailable(&self, operation: &str) -> String {
    super::format_template(self.search_query_unavailable, &[("operation", operation)])
  }
  pub fn cast_no_answer(&self, address: &str) -> String {
    super::format_template(self.cast_no_answer, &[("address", address)])
  }
  pub fn cast_no_tls(&self, address: &str) -> String {
    super::format_template(self.cast_no_tls, &[("address", address)])
  }
  pub fn cast_message_size(&self, size: usize) -> String {
    super::format_template(self.cast_message_size, &[("size", &size.to_string())])
  }
  pub fn cast_no_reply(&self, kind: &str) -> String {
    super::format_template(self.cast_no_reply, &[("kind", kind)])
  }
  pub fn cast_receiver_failed(&self, reason: &str) -> String {
    super::format_template(self.cast_receiver_failed, &[("reason", reason)])
  }
  pub fn cast_login_refused(&self, reply: &str) -> String {
    super::format_template(self.cast_login_refused, &[("reply", reply)])
  }
  pub fn speaker_token_failed(&self, status: &str, error: &str) -> String {
    super::format_template(
      self.speaker_token_failed,
      &[("status", status), ("error", error)],
    )
  }
  pub fn speaker_token_unsupported(&self, kind: &str) -> String {
    super::format_template(self.speaker_token_unsupported, &[("kind", kind)])
  }
  pub fn speaker_login_refused(&self, message: &str, error: &str) -> String {
    super::format_template(
      self.speaker_login_refused,
      &[("message", message), ("error", error)],
    )
  }
  pub fn speaker_not_joined(&self, name: &str) -> String {
    super::format_template(self.speaker_not_joined, &[("name", name)])
  }
  pub fn spotify_api_error(&self, status: &str, body: &str) -> String {
    super::format_template(
      self.spotify_api_error,
      &[("status", status), ("body", body)],
    )
  }
  pub fn local_uri_invalid(&self, error: &str) -> String {
    super::format_template(self.local_uri_invalid, &[("error", error)])
  }
  pub fn local_play_failed(&self, error: &str) -> String {
    super::format_template(self.local_play_failed, &[("error", error)])
  }
  pub fn local_task_failed(&self, error: &str) -> String {
    super::format_template(self.local_task_failed, &[("error", error)])
  }
  pub fn local_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.local_audio_unavailable, &[("error", error)])
  }
  pub fn audio_init_failed(&self, error: &str) -> String {
    super::format_template(self.audio_init_failed, &[("error", error)])
  }
  pub fn local_scan_failed(&self, error: &str) -> String {
    super::format_template(self.local_scan_failed, &[("error", error)])
  }
  pub fn local_scan_task_failed(&self, error: &str) -> String {
    super::format_template(self.local_scan_task_failed, &[("error", error)])
  }
  pub fn local_read_failed(&self, error: &str) -> String {
    super::format_template(self.local_read_failed, &[("error", error)])
  }
  pub fn local_tracks_scan_failed(&self, error: &str) -> String {
    super::format_template(self.local_tracks_scan_failed, &[("error", error)])
  }
  pub fn local_exhausted(&self, message: &str) -> String {
    super::format_template(self.local_exhausted, &[("message", message)])
  }
  pub fn subsonic_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.subsonic_audio_unavailable, &[("error", error)])
  }
  pub fn youtube_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.youtube_audio_unavailable, &[("error", error)])
  }
  pub fn radio_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.radio_audio_unavailable, &[("error", error)])
  }
  pub fn qobuz_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.qobuz_audio_unavailable, &[("error", error)])
  }
  pub fn subsonic_playlists_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_playlists_failed, &[("error", error)])
  }
  pub fn subsonic_playlist_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_playlist_failed, &[("error", error)])
  }
  pub fn subsonic_search_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_search_failed, &[("error", error)])
  }
  pub fn subsonic_download_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_download_failed, &[("error", error)])
  }
  pub fn subsonic_play_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_play_failed, &[("error", error)])
  }
  pub fn subsonic_task_failed(&self, error: &str) -> String {
    super::format_template(self.subsonic_task_failed, &[("error", error)])
  }
  pub fn radio_search_failed(&self, error: &str) -> String {
    super::format_template(self.radio_search_failed, &[("error", error)])
  }
  pub fn radio_open_failed(&self, error: &str) -> String {
    super::format_template(self.radio_open_failed, &[("error", error)])
  }
  pub fn radio_play_failed(&self, error: &str) -> String {
    super::format_template(self.radio_play_failed, &[("error", error)])
  }
  pub fn radio_task_failed(&self, error: &str) -> String {
    super::format_template(self.radio_task_failed, &[("error", error)])
  }
  pub fn radio_playing(&self, name: &str) -> String {
    super::format_template(self.radio_playing, &[("name", name)])
  }
  pub fn youtube_search_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_search_failed, &[("error", error)])
  }
  pub fn youtube_playlists_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_playlists_failed, &[("error", error)])
  }
  pub fn youtube_create_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_create_failed, &[("error", error)])
  }
  pub fn youtube_delete_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_delete_failed, &[("error", error)])
  }
  pub fn youtube_add_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_add_failed, &[("error", error)])
  }
  pub fn youtube_remove_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_remove_failed, &[("error", error)])
  }
  pub fn youtube_download_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_download_failed, &[("error", error)])
  }
  pub fn youtube_task_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_task_failed, &[("error", error)])
  }
  pub fn youtube_created(&self, name: &str) -> String {
    super::format_template(self.youtube_created, &[("name", name)])
  }
  pub fn youtube_deleted(&self, name: &str) -> String {
    super::format_template(self.youtube_deleted, &[("name", name)])
  }
  pub fn youtube_removed(&self, name: &str) -> String {
    super::format_template(self.youtube_removed, &[("name", name)])
  }
  pub fn fetching(&self, name: &str) -> String {
    super::format_template(self.fetching, &[("name", name)])
  }
  pub fn youtube_added(&self, name: &str, playlist: &str) -> String {
    super::format_template(
      self.youtube_added,
      &[("name", name), ("playlist", playlist)],
    )
  }
  pub fn youtube_already_added(&self, name: &str, playlist: &str) -> String {
    super::format_template(
      self.youtube_already_added,
      &[("name", name), ("playlist", playlist)],
    )
  }
  pub fn youtube_decode_failed(&self, error: &str) -> String {
    super::format_template(self.youtube_decode_failed, &[("error", error)])
  }
  pub fn qobuz_login_url(&self, url: &str) -> String {
    super::format_template(self.qobuz_login_url, &[("url", url)])
  }
  pub fn qobuz_login_failed(&self, error: &str) -> String {
    super::format_template(self.qobuz_login_failed, &[("error", error)])
  }
  pub fn qobuz_operation_failed(&self, step: &str, error: &str) -> String {
    super::format_template(
      self.qobuz_operation_failed,
      &[("step", step), ("error", error)],
    )
  }
  // Apple Music exists only in a macOS build with the `apple-music` feature.
  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub fn music_loading(&self, end: usize, total: usize) -> String {
    super::format_template(
      self.music_loading,
      &[("end", &end.to_string()), ("total", &total.to_string())],
    )
  }
  // Apple Music exists only in a macOS build with the `apple-music` feature.
  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub fn music_loaded(&self, count: usize) -> String {
    super::format_template(self.music_loaded, &[("count", &count.to_string())])
  }
  // Apple Music exists only in a macOS build with the `apple-music` feature.
  #[cfg_attr(
    not(all(feature = "apple-music", target_os = "macos")),
    allow(dead_code)
  )]
  pub fn music_status_unavailable(&self, error: &str) -> String {
    super::format_template(self.music_status_unavailable, &[("error", error)])
  }
  pub fn source_playback_unavailable(&self, source: &str) -> String {
    super::format_template(self.source_playback_unavailable, &[("source", source)])
  }
  pub fn queue_skip_disconnected(&self, name: &str) -> String {
    super::format_template(self.queue_skip_disconnected, &[("name", name)])
  }
  pub fn queue_play_failed(&self, name: &str, error: &str) -> String {
    super::format_template(self.queue_play_failed, &[("name", name), ("error", error)])
  }
  pub fn queue_restore_failed(&self, name: &str, error: &str) -> String {
    super::format_template(
      self.queue_restore_failed,
      &[("name", name), ("error", error)],
    )
  }
  pub fn queue_playing(&self, name: &str) -> String {
    super::format_template(self.queue_playing, &[("name", name)])
  }
  pub fn queue_audio_unavailable(&self, error: &str) -> String {
    super::format_template(self.queue_audio_unavailable, &[("error", error)])
  }
  pub fn spotify_login_failed(&self, error: &str) -> String {
    super::format_template(self.spotify_login_failed, &[("error", error)])
  }
  pub fn spotify_rate_wait(&self, seconds: u64) -> String {
    super::format_template(self.spotify_rate_wait, &[("seconds", &seconds.to_string())])
  }
  pub fn spotify_pair(&self, url: &str, code: &str) -> String {
    super::format_template(self.spotify_pair, &[("url", url), ("code", code)])
  }
  pub fn recap_created(&self, path: &str, count: usize) -> String {
    super::format_template(
      self.recap_created,
      &[("path", path), ("count", &count.to_string())],
    )
  }
  pub fn history_failed(&self, error: &str) -> String {
    super::format_template(self.history_failed, &[("error", error)])
  }
  pub fn recap_failed(&self, error: &str) -> String {
    super::format_template(self.recap_failed, &[("error", error)])
  }
  pub fn party_start_failed(&self, error: &str) -> String {
    super::format_template(self.party_start_failed, &[("error", error)])
  }
  pub fn party_join_failed(&self, error: &str) -> String {
    super::format_template(self.party_join_failed, &[("error", error)])
  }
  pub fn party_error(&self, error: &str) -> String {
    super::format_template(self.party_error, &[("error", error)])
  }
  pub fn party_joined(&self, name: &str) -> String {
    super::format_template(self.party_joined, &[("name", name)])
  }
  pub fn party_left(&self, name: &str) -> String {
    super::format_template(self.party_left, &[("name", name)])
  }
  pub fn native_nothing_to_resume(&self, device: &str) -> String {
    super::format_template(self.native_nothing_to_resume, &[("device", device)])
  }
  pub fn native_start_failed(&self, error: &str) -> String {
    super::format_template(self.native_start_failed, &[("error", error)])
  }
  pub fn native_shuffle_failed(&self, error: &str) -> String {
    super::format_template(self.native_shuffle_failed, &[("error", error)])
  }
  pub fn native_resume_failed(&self, error: &str) -> String {
    super::format_template(self.native_resume_failed, &[("error", error)])
  }
  pub fn shuffle_capped(&self, count: usize) -> String {
    super::format_template(self.shuffle_capped, &[("count", &count.to_string())])
  }
  pub fn friend_added_code(&self, code: &str) -> String {
    super::format_template(self.friend_added_code, &[("code", code)])
  }
  pub fn friend_add_failed(&self, error: &str) -> String {
    super::format_template(self.friend_add_failed, &[("error", error)])
  }
  pub fn friend_unfollow_failed(&self, error: &str) -> String {
    super::format_template(self.friend_unfollow_failed, &[("error", error)])
  }
  pub fn liked_check_failed(&self, error: &str) -> String {
    super::format_template(self.liked_check_failed, &[("error", error)])
  }
  pub fn playlist_search_scope(&self, count: u32) -> String {
    super::format_template(self.playlist_search_scope, &[("count", &count.to_string())])
  }
  pub fn playlist_search_matches(&self, count: usize, query: &str, scope: &str) -> String {
    super::format_template(
      self.playlist_search_matches,
      &[
        ("count", &count.to_string()),
        ("query", query),
        ("scope", scope),
      ],
    )
  }
  pub fn playlist_added_named(&self, name: &str) -> String {
    super::format_template(self.playlist_added_named, &[("name", name)])
  }
  pub fn playlist_created(&self, name: &str) -> String {
    super::format_template(self.playlist_created, &[("name", name)])
  }
  pub fn native_recovery_dropped(&self, error: &str) -> String {
    super::format_template(self.native_recovery_dropped, &[("error", error)])
  }
  pub fn native_recovery_failed(&self, error: &str) -> String {
    super::format_template(self.native_recovery_failed, &[("error", error)])
  }
  pub fn sync_failed(&self, error: &str) -> String {
    super::format_template(self.sync_failed, &[("error", error)])
  }
  pub fn sync_link_failed(&self, error: &str) -> String {
    super::format_template(self.sync_link_failed, &[("error", error)])
  }
  pub fn sync_remove_failed(&self, error: &str) -> String {
    super::format_template(self.sync_remove_failed, &[("error", error)])
  }
  pub fn sync_summary(&self, dry: &str, added: usize, removed: usize, unmatched: usize) -> String {
    super::format_template(
      self.sync_summary,
      &[
        ("dry", dry),
        ("added", &added.to_string()),
        ("removed", &removed.to_string()),
        ("unmatched", &unmatched.to_string()),
      ],
    )
  }
  pub fn sync_using_playlist(&self, source: &str, name: &str) -> String {
    super::format_template(
      self.sync_using_playlist,
      &[("source", source), ("name", name)],
    )
  }
  pub fn sync_progress(&self, name: &str, done: usize, total: usize) -> String {
    super::format_template(
      self.sync_progress,
      &[
        ("name", name),
        ("done", &done.to_string()),
        ("total", &total.to_string()),
      ],
    )
  }
  pub fn dj_indexed(&self, tracks: usize, playlists: usize) -> String {
    super::format_template(
      self.dj_indexed,
      &[
        ("tracks", &tracks.to_string()),
        ("playlists", &playlists.to_string()),
      ],
    )
  }
  pub fn dj_index_capped(&self, count: usize) -> String {
    super::format_template(self.dj_index_capped, &[("count", &count.to_string())])
  }
  pub fn dj_queued(&self, count: usize) -> String {
    super::format_template(self.dj_queued, &[("count", &count.to_string())])
  }
  pub fn dj_not_found(&self, count: usize) -> String {
    super::format_template(self.dj_not_found, &[("count", &count.to_string())])
  }
  pub fn dj_owned(&self, count: usize) -> String {
    super::format_template(self.dj_owned, &[("count", &count.to_string())])
  }
  pub fn dj_duplicate(&self, count: usize) -> String {
    super::format_template(self.dj_duplicate, &[("count", &count.to_string())])
  }
  pub fn dj_status(&self, summary: &str) -> String {
    super::format_template(self.dj_status, &[("summary", summary)])
  }
  pub fn dj_library_failed(&self, error: &str) -> String {
    super::format_template(self.dj_library_failed, &[("error", error)])
  }
  pub fn dj_error(&self, backend: &str, error: &str) -> String {
    super::format_template(self.dj_error, &[("backend", backend), ("error", error)])
  }
  pub fn dj_steps_exhausted(&self, count: usize) -> String {
    super::format_template(self.dj_steps_exhausted, &[("count", &count.to_string())])
  }
  pub fn history_plays(&self, count: u64, duration: &str) -> String {
    super::format_template(
      if count == 1 {
        self.history_play
      } else {
        self.history_plays
      },
      &[("count", &count.to_string()), ("duration", duration)],
    )
  }
  pub fn history_track_hits(&self, count: u64, duration: &str) -> String {
    super::format_template(
      if count == 1 {
        self.history_track_hit
      } else {
        self.history_track_hits
      },
      &[("count", &count.to_string()), ("duration", duration)],
    )
  }
  pub fn history_minutes(&self, minutes: u64) -> String {
    super::format_template(self.history_minutes, &[("minutes", &minutes.to_string())])
  }
  pub fn history_hours_minutes(&self, hours: u64, minutes: u64) -> String {
    super::format_template(
      self.history_hours_minutes,
      &[
        ("hours", &hours.to_string()),
        ("minutes", &minutes.to_string()),
      ],
    )
  }
  pub fn setup_no_api_key(&self, agent: &str) -> String {
    super::format_template(self.setup_no_api_key, &[("agent", agent)])
  }
  pub fn setup_keep_command(&self, command: &str) -> String {
    super::format_template(self.setup_keep_command, &[("command", command)])
  }
  pub fn setup_needs_key(&self, variable: &str) -> String {
    super::format_template(self.setup_needs_key, &[("variable", variable)])
  }
  pub fn setup_builtin_default(&self, model: &str) -> String {
    super::format_template(self.setup_builtin_default, &[("model", model)])
  }
  pub fn mcp_status(&self, message: &str) -> String {
    super::format_template(self.mcp_status, &[("message", message)])
  }
  pub fn network_error(&self, error: &str) -> String {
    super::format_template(self.network_error, &[("error", error)])
  }
  pub fn network_rate_error(&self, error: &str) -> String {
    super::format_template(self.network_rate_error, &[("error", error)])
  }
  pub fn monthly_recap_ready(&self, path: &str, count: usize) -> String {
    super::format_template(
      self.monthly_recap_ready,
      &[("path", path), ("count", &count.to_string())],
    )
  }
}

pub const EN: InfraStatusStrings = InfraStatusStrings {
  sync_spotify_not_connected: "Spotify is not connected",
  sync_qobuz_not_logged_in: "Qobuz is not logged in",
  sync_subsonic_not_configured: "No Subsonic server configured (set behavior.subsonic_url)",
  sync_source_not_compiled: "{source} is not compiled into this build (feature `{feature}`)",
  sync_source_unsupported: "{source} playlists cannot be synced",
  sync_source_unreachable: "{source} unreachable: {error}",
  search_query_unavailable: "Search is unavailable: every known {operation} hash is retired",
  made_for_you_unavailable: "Made For You is unavailable: every known browsePage hash is retired",
  made_for_you_missing: "Spotify did not send the Made For You page",
  history_track_hit: "{count} track hit · {duration}",
  history_play: "{count} play · {duration}",
  cast_no_answer: "no answer from {address}",
  cast_no_tls: "no TLS handshake with {address}",
  cast_message_size: "a {size}-byte Cast message",
  cast_malformed: "a malformed Cast message",
  cast_no_reply: "no reply to {kind}",
  cast_receiver_failed: "the Spotify receiver did not start: {reason}",
  cast_client_id_missing: "no clientID in getInfoResponse",
  cast_login_refused: "the speaker refused the login: {reply}",
  cast_access_token_missing: "no accessToken in the device-auth reply",
  speaker_token_failed: "the token exchange failed ({status}): {error}",
  speaker_token_unsupported: "the speaker asks for token type {kind}, which is not supported yet",
  speaker_client_id_missing: "no clientID in getInfo",
  speaker_login_refused: "the speaker refused the login: {message} ({error})",
  speaker_not_joined: "{name} did not join after its login",
  playlist_changed: "The playlist changed since it loaded; open it again and retry",
  spotify_api_error: "Spotify API {status} failed: {body}",
  local_replay_failed: "Cannot replay local file",
  local_no_tracks: "No local tracks to play",
  local_no_folder: "No local music folder configured (set behavior.local_music_path)",
  local_uri_invalid: "Invalid local file URI: {error}",
  local_play_failed: "Cannot play local file: {error}",
  local_task_failed: "Local playback task failed: {error}",
  local_audio_unavailable: "No audio output for local playback: {error}",
  audio_init_failed: "Audio output init failed: {error}",
  local_scan_failed: "Cannot scan music folder: {error}",
  local_scan_task_failed: "Local folder scan failed: {error}",
  local_read_failed: "Cannot read folder: {error}",
  local_tracks_scan_failed: "Local track scan failed: {error}",
  local_exhausted: "{message} — no playable tracks left, stopping",
  subsonic_audio_unavailable: "No audio output for Subsonic playback: {error}",
  youtube_audio_unavailable: "No audio output for YouTube playback: {error}",
  radio_audio_unavailable: "No audio output for radio playback: {error}",
  qobuz_audio_unavailable: "No audio output for Qobuz playback: {error}",
  subsonic_no_tracks: "No Subsonic tracks to play",
  subsonic_uri_invalid: "Invalid Subsonic track URI",
  subsonic_replay_failed: "Cannot replay Subsonic track",
  subsonic_playlists_failed: "Cannot load Subsonic playlists: {error}",
  subsonic_playlist_failed: "Cannot load Subsonic playlist: {error}",
  subsonic_search_failed: "Subsonic search failed: {error}",
  subsonic_download_failed: "Cannot download Subsonic track: {error}",
  subsonic_play_failed: "Cannot play Subsonic track: {error}",
  subsonic_task_failed: "Subsonic playback task failed: {error}",
  subsonic_no_server: "No Subsonic server configured (set behavior.subsonic_url)",
  radio_no_skip: "Live radio: no track skipping",
  radio_empty: "No radio stations configured or saved; search to add some",
  radio_uri_invalid: "Invalid radio URI",
  radio_probe_timeout: "Cannot play radio stream: no recognisable audio format in time.",
  radio_search_failed: "Radio search failed: {error}",
  radio_open_failed: "Cannot open radio stream: {error}",
  radio_play_failed: "Cannot play radio stream: {error}",
  radio_task_failed: "Radio playback task failed: {error}",
  radio_playing: "♪ {name} (live)",
  youtube_playlist_missing: "That YouTube playlist no longer exists",
  youtube_metadata_missing: "Cannot resolve that video's metadata",
  youtube_no_tracks: "No YouTube videos to play",
  youtube_no_live: "Live streams aren't supported on the YouTube source yet",
  youtube_uri_invalid: "Invalid YouTube URI",
  youtube_replay_failed: "Cannot replay YouTube track",
  youtube_search_failed: "YouTube search failed: {error}",
  youtube_playlists_failed: "Cannot load YouTube playlists: {error}",
  youtube_create_failed: "Cannot create YouTube playlist: {error}",
  youtube_delete_failed: "Cannot delete YouTube playlist: {error}",
  youtube_add_failed: "Cannot add video to playlist: {error}",
  youtube_remove_failed: "Cannot remove video from playlist: {error}",
  youtube_download_failed: "Cannot download YouTube audio: {error}",
  youtube_task_failed: "YouTube playback task failed: {error}",
  youtube_created: "Created YouTube playlist \"{name}\"",
  youtube_deleted: "Deleted YouTube playlist \"{name}\"",
  youtube_removed: "Removed \"{name}\" from playlist",
  fetching: "Fetching {name}…",
  youtube_added: "Added \"{name}\" to {playlist}",
  youtube_already_added: "\"{name}\" is already in {playlist}",
  youtube_decode_failed: "Cannot play YouTube audio: {error} (installing ffmpeg fixes most container issues)",
  qobuz_login_expired: "Qobuz: login expired, press `d` and pick Qobuz to log in again",
  qobuz_login_busy: "Qobuz login already in progress...",
  qobuz_browser_login: "Qobuz: open the browser window to log in",
  qobuz_logged_in: "Qobuz: logged in",
  qobuz_no_tracks: "No Qobuz tracks to play",
  qobuz_uri_invalid: "Invalid Qobuz track URI",
  qobuz_replay_failed: "Cannot replay Qobuz track",
  qobuz_login_url: "Open this URL in your browser to log in to Qobuz: {url}",
  qobuz_login_failed: "Qobuz login failed: {error}",
  qobuz_step_constants: "web player constants",
  qobuz_step_login: "login",
  qobuz_step_library: "library",
  qobuz_step_tracks: "tracks",
  qobuz_step_search: "search",
  qobuz_step_stream: "stream",
  qobuz_operation_failed: "Qobuz: {step}: {error}",
  music_busy_playback: "Music is busy; retry the playback request",
  music_wait_pause: "Waiting for Music to pause before switching source",
  music_not_running: "Music is not running; choose a Music track to start it",
  music_shuffle_repeat: "Use Music to change shuffle or repeat",
  music_switch_spotify: "Start a Spotify track to switch playback from Music",
  music_busy: "Music is busy; retry the request",
  music_queue_unsupported: "Apple Music tracks can't be queued",
  music_loading: "Loading Apple Music library: {end}/{total}",
  music_loaded: "Apple Music: loaded {count} items",
  music_status_unavailable: "Music status unavailable: {error}. Retrying in the background.",
  queue_repeat_unsupported: "Repeat does not apply to this source",
  queue_shuffle_unsupported: "Shuffle does not apply to this source",
  queue_no_uri: "Skipped a queued track with no URI",
  queue_finished: "Queue finished",
  audio_disconnected: "Audio output device disconnected.",
  radio_not_resumed: "Playback is paused - radio not resumed.",
  source_playback_unavailable: "{source} playback isn't available in this build",
  queue_skip_disconnected: "Native streaming isn't connected; skipped \"{name}\"",
  queue_play_failed: "Cannot play {name}: {error}",
  queue_restore_failed: "Cannot restore {name}: {error}",
  queue_playing: "♪ {name} (queue)",
  queue_audio_unavailable: "No audio output for queue playback: {error}",
  spotify_not_connected: "Spotify not connected. Press `d` and pick Spotify to log in.",
  playlist_id_invalid: "Invalid playlist identifier.",
  music_requires_macos: "Apple Music requires macOS and the apple-music feature",
  spotify_not_compiled: "This build of spotatui has no Spotify support",
  spotify_connected: "Spotify connected. Restart spotatui to enable native playback.",
  party_disconnected: "Listening Party ended: Spotify is not connected.",
  party_ended: "Party ended",
  spotify_login_timeout: "Spotify login timed out. Press `d` and pick Spotify to try again.",
  spotify_login_failed: "Spotify login failed: {error}",
  spotify_rate_wait: "Spotify rate limit: waiting {seconds}s",
  spotify_pair: "Log in to Spotify: open {url} and confirm the code {code}",
  recap_created: "Listening recap generated at {path} ({count} listens)",
  history_failed: "failed to load listening history: {error}",
  recap_failed: "Failed to generate recap: {error}",
  party_start_failed: "Failed to start party: {error}",
  party_join_failed: "Failed to join party: {error}",
  party_error: "Party: {error}",
  party_joined: "{name} joined the party",
  party_left: "{name} left the party",
  native_reconnecting: "Reconnecting native streaming…",
  native_settling: "Native streaming is still settling…",
  shuffle_not_applied: "Playback started; couldn't apply shuffle.",
  spotify_rate_retry: "Spotify rate limit hit. Retrying automatically; please wait a few seconds.",
  spotify_poll_retry: "Temporary Spotify network error while polling playback; retrying automatically.",
  spotify_server_retry: "Spotify server temporarily unavailable (5xx); retrying automatically.",
  spotify_poll_rejected: "Spotify keeps rejecting the playback poll; retrying automatically.",
  native_resuming: "Resuming the last playback here…",
  native_reconnect_resume: "Reconnecting native streaming; playback will resume.",
  native_connection_restored: "Native connection restored; restoring playback.",
  native_no_state: "Native connection recovered, but there was no playback state to restore.",
  native_restore_retry: "Native playback restore failed; reconnecting again.",
  other_source_playing: "Another source owns playback",
  spotify_playing_elsewhere: "Playback remains on another Spotify device.",
  queue_added: "Added to queue",
  queue_load_failed: "Could not load queue (no active device?)",
  native_nothing_to_resume: "No playback to resume on {device}.",
  native_start_failed: "Failed to start native playback: {error}",
  native_shuffle_failed: "Failed to apply shuffle: {error}",
  native_resume_failed: "Failed to resume playback: {error}",
  shuffle_context_failed: "Shuffle: failed to load the context's track list",
  shuffle_capped: "Large context: shuffling the first {count} tracks",
  spotify_profile_retry: "Spotify rate limit hit while loading profile. Retrying automatically.",
  friend_added: "Added friend",
  friend_unfollowed: "Unfollowed",
  friend_added_code: "Added friend (code: {code})",
  friend_add_failed: "Could not add friend: {error}",
  friend_unfollow_failed: "Failed to unfollow: {error}",
  playlist_external_unavailable: "Spotify Development Mode blocks playlist contents owned by another user. Only playlists you own or collaborate on are available.",
  liked_check_failed: "Could not check liked track state: {error}",
  playlist_search_scope: " in the first {count} tracks",
  playlist_search_matches: "{count} playlist tracks match \"{query}\"{scope}",
  playlist_refresh_partial: "Playlist refresh incomplete; kept the previous list.",
  playlist_added: "Added to playlist",
  playlist_removed: "Removed from playlist",
  playlist_created_no_id: "Playlist created but could not get its ID",
  playlist_added_named: "Added to {name}",
  playlist_created: "Playlist \"{name}\" created!",
  native_recovered: "Native streaming recovered.",
  native_lost_reconnecting: "Native streaming connection lost; reconnecting.",
  native_reconnected: "Native streaming connection recovered.",
  native_restored: "Native playback restored.",
  native_restored_paused: "Native playback restored in paused state.",
  native_skip_unavailable: "Couldn't play this track natively (unavailable or blocked); skipping.",
  native_many_unavailable: "Several tracks in a row couldn't be played natively, so playback was stopped. They may be unavailable on your account or region. Press 'd' to switch to an official Spotify Connect device.",
  native_recovery_dropped: "Native recovery failed; playback request dropped: {error}",
  native_recovery_failed: "Native recovery failed: {error}",
  native_lost_recovering: "Native streaming connection lost; attempting recovery.",
  native_stalled: "Native streaming stalled; attempting recovery.",
  native_stopped: "Native streaming stopped.",
  native_moved: "Playback moved to another Spotify device.",
  native_disconnected: "Native streaming disconnected; attempting recovery.",
  native_stopped_recovering: "Native streaming stopped; attempting recovery.",
  sync_busy: "Playlist sync already running",
  sync_no_links: "Playlist sync: no links",
  sync_dry_run: " (dry run)",
  sync_removed: "Playlist link removed; the mirror playlists stay",
  sync_failed: "Playlist sync failed: {error}",
  sync_link_failed: "Playlist link failed: {error}",
  sync_remove_failed: "Removing the link failed: {error}",
  sync_summary: "Playlist sync{dry}: {added} added, {removed} removed, {unmatched} unmatched",
  sync_using_playlist: "Using the existing {source} playlist \"{name}\"",
  sync_progress: "Playlist sync: {name} {done}/{total}",
  dj_indexed: "Indexed {tracks} track(s) across {playlists} playlist(s)",
  dj_index_capped: " (stopped at {count}; the rest is not filtered)",
  dj_queued: "queued {count}",
  dj_not_found: "{count} not found",
  dj_owned: "{count} already yours",
  dj_duplicate: "{count} already queued",
  dj_status: "DJ: {summary}",
  dj_library_failed: "DJ: could not read your playlists ({error}); recommending unfiltered",
  dj_topped_up: "DJ: topped up the queue",
  dj_nothing_queued: "DJ: nothing was queued that time. Ask again, or try a different direction.",
  dj_error: "DJ error ({backend}): {error}",
  dj_steps_exhausted: "Stopped after {count} steps. Ask again if that is not where you wanted to end up.",
  history_plays: "{count} plays · {duration}",
  history_track_hits: "{count} track hits · {duration}",
  history_minutes: "{minutes}m",
  history_hours_minutes: "{hours}h {minutes}m",
  setup_no_api_key: "{agent} (no API key)",
  setup_keep_command: "{command} (keep)",
  setup_anthropic: "anthropic (API key)",
  setup_openai: "openai_compat (local or self-hosted)",
  setup_own_command: "your own command, run exactly as written",
  setup_not_installed: " · not on PATH",
  setup_anthropic_billing: "Anthropic API, pay per token",
  setup_needs_key: "needs {variable} (or behavior.dj_api_key)",
  setup_openai_endpoint: "any OpenAI-compatible endpoint, e.g. Ollama on localhost",
  setup_claude_plan: "uses your Claude Pro/Max plan",
  setup_codex_plan: "uses your ChatGPT plan",
  setup_agy_plan: "uses your Antigravity plan",
  setup_copilot_plan: "uses your GitHub Copilot plan",
  setup_opencode_plan: "uses whatever provider opencode is logged into",
  setup_gemini_legacy: "legacy Gemini CLI, superseded by agy",
  setup_agent_subscription: "uses that CLI's own subscription",
  setup_keep_as_is: "Keep this command as it is",
  setup_own_model_note: "spotatui adds no model flag to a command it does not own",
  setup_current: "currently configured",
  setup_cli_default: "Use the CLI's default",
  setup_no_model_flag: "no model flag",
  setup_builtin_default: "Use the built-in default ({model})",
  setup_custom: "Custom…",
  setup_type_provider_model: "type a model name, as provider/model",
  setup_type_model: "type a model name",
  setup_haiku_note: "cheapest, easiest on a Pro plan",
  setup_sonnet_note: "balanced",
  setup_opus_note: "heavy, a Pro plan hits its limit fast",
  setup_fable_note: "heaviest, hits it faster still",
  setup_cheapest: "cheapest",
  setup_heaviest_gemini: "heaviest Gemini",
  setup_heaviest_all: "heaviest of all",
  setup_price_haiku: "$1/$5 per MTok, 200K ctx",
  setup_price_sonnet: "$3/$15 per MTok, 1M ctx",
  setup_price_opus: "$5/$25 per MTok, 1M ctx",
  setup_price_fable: "$10/$50 per MTok, 1M ctx",
  history_week: "Last 7 Days",
  history_month: "Last 30 Days",
  history_all: "All Time",
  history_this_month: "This Month",
  history_this_year: "This Year",
  mcp_status: "MCP: {message}",
  network_error: "{error}",
  network_rate_error: "{error}",
  monthly_recap_ready: "Monthly listening recap ready at {path} ({count} listens)",
  queue_local_source: "Local",
  unnamed_folder: "Unnamed Folder",
  local_root_folder: "(root)",
  local_unknown_track: "Unknown Track",
};

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANS: InfraStatusStrings = InfraStatusStrings { ..EN };

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: InfraStatusStrings = InfraStatusStrings { ..EN };
