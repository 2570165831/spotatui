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
pub const ZH_HANS: OnboardingStrings = OnboardingStrings {
  welcome_sources: "\n欢迎使用 spotatui！请选择音源：",
  welcome_source: "\n欢迎使用 spotatui！请选择音源：\n",
  switch_sources: "随时可在 `d` 菜单中添加或切换音源。",
  picker_keys: "↑/↓ 移动 · space 选择 · enter 确认 · esc 跳过",
  choose_template: "\n请选择（1-{max}）： ",
  invalid_choice_template: "选择无效。请输入 1 到 {max} 之间的数字。",
  note_spotify: "需要登录",
  note_youtube: "免费，需要 yt-dlp 程序",
  note_subsonic: "免费，需要 Subsonic/Navidrome 服务器",
  note_free: "免费",
  note_qobuz: "需要付费订阅，通过浏览器登录",
  note_apple_music: "macOS“音乐”应用，需要自动化权限",
  config_save_failed_template: "无法保存 {path}：{error}",
  setting_up_sources: "\n正在设置其他音源，随后登录 Spotify…\n",
  starting_source_template:
    "\n正在以 {source} 为音源启动 spotatui。随时按 `d` 切换音源或登录 Spotify。\n",
  qobuz_setup: "\nQobuz 设置：spotatui 将通过浏览器登录（需要付费订阅 Qobuz）。",
  fetching_qobuz_constants: "正在获取 Qobuz 网页播放器参数… ",
  ok: "完成",
  failed_template: "失败：{error}",
  qobuz_retry: "在应用中按 `d` 并选择 Qobuz 即可重试。",
  qobuz_login_start_failed_template: "无法开始 Qobuz 登录：{error}",
  opening_browser: "\n正在尝试用浏览器打开以下网址：",
  browser_open_failed_template: "无法自动打开浏览器：{error}",
  open_browser_manually: "请在浏览器中手动打开上方网址。",
  qobuz_waiting: "正在等待 Qobuz 登录完成…",
  qobuz_login_failed_template: "Qobuz 登录失败：{error}",
  qobuz_logged_in: "已登录 Qobuz。",
  testing_stream: "正在测试流媒体会话… ",
  subsonic_setup: "\nSubsonic / Navidrome 设置：",
  server_url: "服务器地址（例如 https://demo.navidrome.org）",
  username: "用户名",
  password: "密码",
  testing_connection: "正在测试连接… ",
  saved_anyway_template: "不过设置已经保存。如有需要，请修改 {path} 中的信息后重新启动。",
  session_only: "这些信息仅在本次会话中保留。",
  checking_ytdlp: "\n正在检查 yt-dlp… ",
  ytdlp_found_template: "已找到（{version}）",
  ytdlp_not_found: "未找到",
  ytdlp_required: "播放 YouTube 需要 PATH 中有 `yt-dlp` 程序。",
  ytdlp_install: "请先安装（例如使用 `pipx install yt-dlp` 或发行版软件包），再重新启动。",
  ytdlp_custom_path_template: "若程序位于自定义路径，请在 {path} 中设置 behavior.ytdlp_path。",
  local_music_path_template: "\n将从此文件夹读取本地文件：{path}",
  change_local_music_path_template:
    "（如需使用其他文件夹，请修改 {path} 中的 behavior.local_music_path。）",
  no_local_music_path: "\n未自动找到音乐文件夹。",
  set_local_music_path_template: "请在 {path} 中设置 behavior.local_music_path。",
  required: "  （必填）",
  spotify_login_template: "登录 Spotify：打开 {url} 并确认配对码 {code}\n",
  spotify_logged_in: "已登录 Spotify。\n",
  ..EN
};

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: OnboardingStrings = OnboardingStrings {
  welcome_sources: "\n歡迎使用 spotatui！請選擇音樂來源：",
  welcome_source: "\n歡迎使用 spotatui！請選擇音樂來源：\n",
  switch_sources: "隨時可在 `d` 選單中新增或切換來源。",
  picker_keys: "↑/↓ 移動 · space 選取 · enter 確認 · esc 略過",
  choose_template: "\n請選擇（1-{max}）： ",
  invalid_choice_template: "選擇無效。請輸入 1 到 {max} 之間的數字。",
  note_spotify: "需要登入",
  note_youtube: "免費，需要 yt-dlp 程式",
  note_subsonic: "免費，需要 Subsonic/Navidrome 伺服器",
  note_free: "免費",
  note_qobuz: "需要付費訂閱，透過瀏覽器登入",
  note_apple_music: "macOS「音樂」應用程式，需要自動化權限",
  config_save_failed_template: "無法儲存 {path}：{error}",
  setting_up_sources: "\n正在設定其他來源，接著登入 Spotify…\n",
  starting_source_template:
    "\n正在以 {source} 為來源啟動 spotatui。隨時按 `d` 切換來源或登入 Spotify。\n",
  qobuz_setup: "\nQobuz 設定：spotatui 將透過瀏覽器登入（需要付費訂閱 Qobuz）。",
  fetching_qobuz_constants: "正在取得 Qobuz 網頁播放器參數… ",
  ok: "完成",
  failed_template: "失敗：{error}",
  qobuz_retry: "在應用程式中按 `d` 並選擇 Qobuz 即可重試。",
  qobuz_login_start_failed_template: "無法開始 Qobuz 登入：{error}",
  opening_browser: "\n正在嘗試以瀏覽器開啟以下網址：",
  browser_open_failed_template: "無法自動開啟瀏覽器：{error}",
  open_browser_manually: "請在瀏覽器中手動開啟上方網址。",
  qobuz_waiting: "正在等待 Qobuz 登入完成…",
  qobuz_login_failed_template: "Qobuz 登入失敗：{error}",
  qobuz_logged_in: "已登入 Qobuz。",
  testing_stream: "正在測試串流工作階段… ",
  subsonic_setup: "\nSubsonic / Navidrome 設定：",
  server_url: "伺服器位址（例如 https://demo.navidrome.org）",
  username: "使用者名稱",
  password: "密碼",
  testing_connection: "正在測試連線… ",
  saved_anyway_template: "不過設定已經儲存。如有需要，請修改 {path} 中的資訊後重新啟動。",
  session_only: "這些資訊僅在本次工作階段中保留。",
  checking_ytdlp: "\n正在檢查 yt-dlp… ",
  ytdlp_found_template: "已找到（{version}）",
  ytdlp_not_found: "未找到",
  ytdlp_required: "播放 YouTube 需要 PATH 中有 `yt-dlp` 程式。",
  ytdlp_install: "請先安裝（例如使用 `pipx install yt-dlp` 或發行版套件），再重新啟動。",
  ytdlp_custom_path_template: "若程式位於自訂路徑，請在 {path} 中設定 behavior.ytdlp_path。",
  local_music_path_template: "\n將從此資料夾讀取本機檔案：{path}",
  change_local_music_path_template:
    "（如需使用其他資料夾，請修改 {path} 中的 behavior.local_music_path。）",
  no_local_music_path: "\n未自動找到音樂資料夾。",
  set_local_music_path_template: "請在 {path} 中設定 behavior.local_music_path。",
  required: "  （必填）",
  spotify_login_template: "登入 Spotify：開啟 {url} 並確認配對碼 {code}\n",
  spotify_logged_in: "已登入 Spotify。\n",
  ..EN
};

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
