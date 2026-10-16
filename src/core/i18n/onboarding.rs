//! First-run source selection, setup and login messages.

use crate::core::source::Source;

use super::format_template;

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub struct OnboardingStrings {
  pub welcome_sources: &'static str,
  pub welcome_source: &'static str,
  pub switch_sources: &'static str,
  pub picker_keys: &'static str,
  pub choose_template: &'static str,
  pub invalid_choice_template: &'static str,
  pub note_spotify: &'static str,
  pub note_youtube: &'static str,
  pub note_subsonic: &'static str,
  pub note_free: &'static str,
  pub note_qobuz: &'static str,
  pub note_apple_music: &'static str,
  pub config_save_failed_template: &'static str,
  pub setting_up_sources: &'static str,
  pub starting_source_template: &'static str,
  pub qobuz_setup: &'static str,
  pub fetching_qobuz_constants: &'static str,
  pub ok: &'static str,
  pub failed_template: &'static str,
  pub qobuz_retry: &'static str,
  pub qobuz_login_start_failed_template: &'static str,
  pub opening_browser: &'static str,
  pub browser_open_failed_template: &'static str,
  pub open_browser_manually: &'static str,
  pub qobuz_waiting: &'static str,
  pub qobuz_login_failed_template: &'static str,
  pub qobuz_logged_in: &'static str,
  pub testing_stream: &'static str,
  pub subsonic_setup: &'static str,
  pub server_url: &'static str,
  pub username: &'static str,
  pub password: &'static str,
  pub testing_connection: &'static str,
  pub saved_anyway_template: &'static str,
  pub session_only: &'static str,
  pub checking_ytdlp: &'static str,
  pub ytdlp_found_template: &'static str,
  pub ytdlp_not_found: &'static str,
  pub ytdlp_required: &'static str,
  pub ytdlp_install: &'static str,
  pub ytdlp_custom_path_template: &'static str,
  pub local_music_path_template: &'static str,
  pub change_local_music_path_template: &'static str,
  pub no_local_music_path: &'static str,
  pub set_local_music_path_template: &'static str,
  pub required: &'static str,
  pub spotify_login_template: &'static str,
  pub spotify_logged_in: &'static str,
}

pub const EN: OnboardingStrings = OnboardingStrings {
  welcome_sources: "\nWelcome to spotatui! Choose your music sources:",
  welcome_source: "\nWelcome to spotatui! Choose your music source:\n",
  switch_sources: "You can add or switch sources anytime from the `d` menu.",
  picker_keys: "↑/↓ move · space select · enter confirm · esc skip",
  choose_template: "\nChoose (1-{max}): ",
  invalid_choice_template: "Invalid choice. Please enter a number between 1 and {max}.",
  note_spotify: "needs login",
  note_youtube: "free, needs the yt-dlp binary",
  note_subsonic: "free, needs a Subsonic/Navidrome server",
  note_free: "free",
  note_qobuz: "paid subscription, logs in through the browser",
  note_apple_music: "macOS Music app, needs Automation permission",
  config_save_failed_template: "Could not save {path}: {error}",
  setting_up_sources: "\nSetting up your other sources, then we'll log in to Spotify...\n",
  starting_source_template: "\nStarting spotatui with {source} as your source. Press `d` anytime to switch or to log in to Spotify.\n",
  qobuz_setup: "\nQobuz setup: spotatui logs in through your browser (a paid Qobuz subscription is needed).",
  fetching_qobuz_constants: "Fetching the Qobuz web player constants... ",
  ok: "OK",
  failed_template: "failed: {error}",
  qobuz_retry: "Press `d` in the app and pick Qobuz to try again.",
  qobuz_login_start_failed_template: "Qobuz login could not start: {error}",
  opening_browser: "\nAttempting to open this URL in your browser:",
  browser_open_failed_template: "Failed to open browser automatically: {error}",
  open_browser_manually: "Please manually open the URL above in your browser.",
  qobuz_waiting: "Waiting for the Qobuz login to complete...",
  qobuz_login_failed_template: "Qobuz login failed: {error}",
  qobuz_logged_in: "Logged in to Qobuz.",
  testing_stream: "Testing the stream session... ",
  subsonic_setup: "\nSubsonic / Navidrome setup:",
  server_url: "Server URL (e.g. https://demo.navidrome.org)",
  username: "Username",
  password: "Password",
  testing_connection: "Testing connection... ",
  saved_anyway_template: "Saved anyway. Fix the details in {path} and relaunch if needed.",
  session_only: "The details are kept for this session only.",
  checking_ytdlp: "\nChecking for yt-dlp... ",
  ytdlp_found_template: "found ({version})",
  ytdlp_not_found: "not found",
  ytdlp_required: "YouTube playback needs the `yt-dlp` binary on your PATH.",
  ytdlp_install: "Install it (e.g. `pipx install yt-dlp` or your distro package) and relaunch.",
  ytdlp_custom_path_template: "If it lives at a custom path, set behavior.ytdlp_path in {path}.",
  local_music_path_template: "\nLocal files will be read from: {path}",
  change_local_music_path_template: "(Change behavior.local_music_path in {path} to use another folder.)",
  no_local_music_path: "\nNo music folder was detected automatically.",
  set_local_music_path_template: "Set behavior.local_music_path in {path}.",
  required: "  (required)",
  spotify_login_template: "Log in to Spotify: open {url} and confirm the code {code}\n",
  spotify_logged_in: "Logged in to Spotify.\n",
};

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANS: OnboardingStrings = OnboardingStrings { ..EN };

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: OnboardingStrings = OnboardingStrings { ..EN };

// The serialized table is shared by all frontends; individual setup flows
// are compiled only when their source or the terminal frontend is enabled.
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
impl OnboardingStrings {
  pub fn source_note(&self, source: Source) -> &'static str {
    match source {
      Source::Spotify => self.note_spotify,
      Source::YouTube => self.note_youtube,
      Source::Subsonic => self.note_subsonic,
      Source::Radio | Source::Local => self.note_free,
      Source::Qobuz => self.note_qobuz,
      Source::AppleMusic => self.note_apple_music,
    }
  }

  pub fn choose(&self, max: usize) -> String {
    format_template(self.choose_template, &[("max", &max.to_string())])
  }

  pub fn invalid_choice(&self, max: usize) -> String {
    format_template(self.invalid_choice_template, &[("max", &max.to_string())])
  }

  pub fn config_save_failed(&self, path: &str, error: &str) -> String {
    format_template(
      self.config_save_failed_template,
      &[("path", path), ("error", error)],
    )
  }

  pub fn starting_source(&self, source: &str) -> String {
    format_template(self.starting_source_template, &[("source", source)])
  }

  pub fn failed(&self, error: &str) -> String {
    format_template(self.failed_template, &[("error", error)])
  }

  pub fn qobuz_login_start_failed(&self, error: &str) -> String {
    format_template(self.qobuz_login_start_failed_template, &[("error", error)])
  }

  pub fn browser_open_failed(&self, error: &str) -> String {
    format_template(self.browser_open_failed_template, &[("error", error)])
  }

  pub fn qobuz_login_failed(&self, error: &str) -> String {
    format_template(self.qobuz_login_failed_template, &[("error", error)])
  }

  pub fn saved_anyway(&self, path: &str) -> String {
    format_template(self.saved_anyway_template, &[("path", path)])
  }

  pub fn ytdlp_found(&self, version: &str) -> String {
    format_template(self.ytdlp_found_template, &[("version", version)])
  }

  pub fn ytdlp_custom_path(&self, path: &str) -> String {
    format_template(self.ytdlp_custom_path_template, &[("path", path)])
  }

  pub fn local_music_path(&self, path: &str) -> String {
    format_template(self.local_music_path_template, &[("path", path)])
  }

  pub fn change_local_music_path(&self, path: &str) -> String {
    format_template(self.change_local_music_path_template, &[("path", path)])
  }

  pub fn set_local_music_path(&self, path: &str) -> String {
    format_template(self.set_local_music_path_template, &[("path", path)])
  }

  pub fn spotify_login(&self, url: &str, code: &str) -> String {
    format_template(self.spotify_login_template, &[("url", url), ("code", code)])
  }
}
