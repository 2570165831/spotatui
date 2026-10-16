//! Startup and routing messages, shared by both frontends.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub struct RuntimeStatusStrings {
  pub local_file_missing: &'static str,
  pub mcp_listening: &'static str,
  pub mcp_start_failed: &'static str,
  pub native_start_failed: &'static str,
  pub premium_required: &'static str,
  pub native_offline: &'static str,
  pub saved_device_unavailable: &'static str,
  pub apple_music_needs_macos: &'static str,
  pub feature_missing: &'static str,
  pub spotify_login_failed: &'static str,
  pub song_counter_title: &'static str,
  pub song_counter_body: &'static str,
  pub song_counter_question: &'static str,
  pub song_counter_save_failed: &'static str,
  pub song_counter_thanks: &'static str,
  pub song_counter_opted_out: &'static str,
  pub forgetting_spotify_login: &'static str,
}

pub const EN: RuntimeStatusStrings = RuntimeStatusStrings {
  local_file_missing: "Cannot find local file: {path}",
  mcp_listening: "MCP server listening on 127.0.0.1:{port}",
  mcp_start_failed: "MCP server failed to start: {error}",
  native_start_failed: "Native streaming didn't start; using Spotify Connect for now. Restart spotatui to reconnect native playback.",
  premium_required: "Spotify Premium is required for playback",
  native_offline: "Could not connect to Spotify: native streaming disabled",
  saved_device_unavailable: "Saved device unavailable; using {device}",
  apple_music_needs_macos: "Apple Music playback needs macOS",
  feature_missing: "This spotatui was built without the `{feature}` feature, so nothing can play that.",
  spotify_login_failed: "Spotify login failed: {error}. Press `d` and pick Spotify to try again.",
  song_counter_title: "Global Song Counter",
  song_counter_body: "\nspotatui can contribute to a global counter showing total\nsongs played by all users worldwide.\n\nPrivacy: This feature is completely anonymous.\n• No personal information is collected\n• No song names, artists, or listening history\n• Only a simple increment once a song has played for 30 seconds",
  song_counter_question: "\nWould you like to participate? (Y/n): ",
  song_counter_save_failed: "Could not save your answer to {path} (read-only config?). Set `enable_global_song_count` under `behavior:` there to stop this prompt.\n",
  song_counter_thanks: "Thank you for participating!\n",
  song_counter_opted_out: "Opted out. You can change this anytime in Settings -> Behavior.\n",
  forgetting_spotify_login: "\nForgetting the saved Spotify login...",
};

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANS: RuntimeStatusStrings = RuntimeStatusStrings { ..EN };

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: RuntimeStatusStrings = RuntimeStatusStrings { ..EN };

// The full table is shipped on every feature leg; some producers are optional.
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
impl RuntimeStatusStrings {
  pub fn song_counter_save_failed(&self, path: &str) -> String {
    super::format_template(self.song_counter_save_failed, &[("path", path)])
  }
  pub fn local_file_missing(&self, path: &str) -> String {
    super::format_template(self.local_file_missing, &[("path", path)])
  }
  pub fn mcp_listening(&self, port: u16) -> String {
    super::format_template(self.mcp_listening, &[("port", &port.to_string())])
  }
  pub fn mcp_start_failed(&self, error: &str) -> String {
    super::format_template(self.mcp_start_failed, &[("error", error)])
  }
  pub fn saved_device_unavailable(&self, device: &str) -> String {
    super::format_template(self.saved_device_unavailable, &[("device", device)])
  }
  pub fn feature_missing(&self, feature: &str) -> String {
    super::format_template(self.feature_missing, &[("feature", feature)])
  }
  pub fn spotify_login_failed(&self, error: &str) -> String {
    super::format_template(self.spotify_login_failed, &[("error", error)])
  }
}
