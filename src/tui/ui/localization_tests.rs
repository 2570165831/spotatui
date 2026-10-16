//! Rendering regressions shared by the sidebar/help and main-screen migrations.

use super::{help::padded_help_cell, library, popups};
use crate::core::app::App;
use crate::core::geometry::Viewport;
use crate::core::i18n::Language;
use crate::core::source::Source;
use crate::core::test_helpers::playlist_info;
use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, Frame, Terminal};
use unicode_width::UnicodeWidthStr;

type AreaRenderer = fn(&mut Frame<'_>, &App, Rect);

fn localized_app(language: Language, connected: bool) -> App {
  let mut app = if connected {
    App::default_connected()
  } else {
    App::default()
  };
  app.user_config.behavior.language = language;
  app.refresh_language();
  app
}

fn render_area(app: &App, width: u16, height: u16, draw: AreaRenderer) -> Buffer {
  let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
  terminal
    .draw(|frame| draw(frame, app, frame.area()))
    .unwrap();
  terminal.backend().buffer().clone()
}

fn render_help(app: &mut App, width: u16, height: u16) -> Buffer {
  app.view.size = Viewport { width, height };
  popups::ensure_help_menu_model(app);
  let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
  terminal
    .draw(|frame| popups::draw_help_menu(frame, app))
    .unwrap();
  terminal.backend().buffer().clone()
}

/// Ratatui reserves a blank continuation cell for wide glyphs; skip it so the
/// extracted text contains the actual Chinese words, without synthetic spaces.
fn buffer_text(buffer: &Buffer) -> String {
  let mut text = String::new();
  for y in buffer.area().top()..buffer.area().bottom() {
    let mut x = buffer.area().left();
    while x < buffer.area().right() {
      let symbol = buffer.cell((x, y)).unwrap().symbol();
      text.push_str(symbol);
      x = x.saturating_add(symbol.width().max(1) as u16);
    }
    text.push('\n');
  }
  text
}

#[test]
fn sidebar_preserves_user_names_that_look_like_template_placeholders() {
  for language in [Language::ZhHans, Language::ZhHant] {
    let mut app = localized_app(language, false);
    app.active_source = Source::Local;
    *app.local_playlists_mut() = vec![playlist_info("folder", "夜航{key}Mix", "User", false)];
    for width in [24, 40] {
      let text = buffer_text(&render_area(&app, width, 6, library::draw_playlist_block));
      assert!(
        text.contains("夜航{key}Mix"),
        "{language:?}/{width}: {text}"
      );
    }
  }
}

#[test]
fn synthetic_source_rows_follow_live_language_changes_without_renaming_user_playlists() {
  let mut app = localized_app(Language::En, false);
  let mut apple_library = playlist_info("synthetic", "All songs", "Music", false);
  apple_library.uri = "applemusic:library".to_string();
  let mut apple_user = playlist_info("user", "All songs", "Music", false);
  apple_user.uri = "applemusic:playlist:user".to_string();
  app.append_apple_music_playlists(vec![apple_library, apple_user]);
  let mut qobuz_favorites = playlist_info("synthetic", "Favorite tracks", "User", false);
  qobuz_favorites.uri = "qobuz:favorites:tracks".to_string();
  let mut qobuz_user = playlist_info("user", "Favorite tracks", "User", false);
  qobuz_user.uri = "qobuz:playlist:user".to_string();
  *app.qobuz_playlists_mut() = vec![qobuz_favorites, qobuz_user];

  for language in [Language::ZhHans, Language::ZhHant, Language::En] {
    app.user_config.behavior.language = language;
    app.refresh_language();
    app.active_source = Source::AppleMusic;
    let text = buffer_text(&render_area(&app, 40, 8, library::draw_playlist_block));
    assert!(text.contains(app.strings().sidebar.all_songs), "{text}");
    assert!(
      text.contains("All songs"),
      "real playlist was renamed: {text}"
    );
    app.active_source = Source::Qobuz;
    let text = buffer_text(&render_area(&app, 40, 8, library::draw_playlist_block));
    assert!(
      text.contains(app.strings().sidebar.favorite_tracks),
      "{text}"
    );
    assert!(
      text.contains("Favorite tracks"),
      "real playlist was renamed: {text}"
    );
  }
  assert_eq!(app.apple_music_playlists()[0].name, "All songs");
  assert_eq!(app.qobuz_playlists()[0].name, "Favorite tracks");
}

#[test]
fn switching_language_rebuilds_the_help_cache_without_changing_keys() {
  let mut app = localized_app(Language::En, false);
  render_help(&mut app, 80, 15);
  let english_header = app.view.help_menu_model.as_ref().unwrap().header.clone();
  let keys = app.user_config.keys.clone();
  for language in [Language::ZhHans, Language::ZhHant, Language::En] {
    app.user_config.behavior.language = language;
    app.refresh_language();
    assert!(
      app.view.help_menu_model.is_none(),
      "stale {language:?} model"
    );
    let text = buffer_text(&render_help(&mut app, 80, 15));
    assert!(
      text.contains(app.strings().help.column_description),
      "{text}"
    );
    assert!(text.contains(app.strings().help.title), "{text}");
    assert!(app.user_config.keys == keys);
    if language == Language::En {
      assert_eq!(
        app.view.help_menu_model.as_ref().unwrap().header,
        english_header
      );
    }
  }
}

#[test]
fn help_cell_truncation_preserves_english_and_counts_chinese_display_columns() {
  for width in 0..48 {
    for english in ["", "Description", "<Ctrl+f>", "Increase volume by 10%"] {
      let old = if width == 0 {
        String::new()
      } else if english.chars().count() > width {
        format!("{}…", english.chars().take(width - 1).collect::<String>())
      } else {
        english.to_string()
      };
      assert_eq!(padded_help_cell(english, width), format!("{old:<width$}"));
    }
    for chinese in ["随机播放", "循環播放", "播放清單 Mix", "音量增加 10%"] {
      assert_eq!(padded_help_cell(chinese, width).width(), width);
    }
  }
  assert_eq!(padded_help_cell("音量增加", 1), "…");
  assert_eq!(padded_help_cell("音量增加", 2), "… ");
  assert_eq!(padded_help_cell("音量增加", 4), "音… ");
}

#[test]
fn cached_generated_names_change_in_both_terminal_library_views() {
  use crate::core::app::{PlaylistFolder, PlaylistFolderItem};
  use crate::core::plugin_api::GeneratedPlaylistName;
  let mut app = localized_app(Language::En, false);
  app.active_source = Source::Local;
  let mut root = playlist_info("root", "(root)", "local", false);
  root.generated_name = Some(GeneratedPlaylistName::LocalRoot);
  *app.local_playlists_mut() = vec![root, playlist_info("literal", "(root)", "local", false)];
  for language in [Language::En, Language::ZhHans, Language::ZhHant] {
    app.user_config.behavior.language = language;
    app.refresh_language();
    for draw in [
      library::draw_playlist_block as AreaRenderer,
      super::tables::draw_local_browser,
    ] {
      let text = buffer_text(&render_area(&app, 60, 15, draw));
      assert!(
        text.contains(app.strings().status.infra.local_root_folder),
        "{text}"
      );
      assert!(text.contains("(root)"), "{text}");
    }
  }
  app.active_source = Source::Spotify;
  *app.playlist_folder_items_mut() = vec![
    PlaylistFolderItem::Folder(PlaylistFolder {
      name: "Unnamed Folder".into(),
      generated_name: true,
      current_id: 0,
      target_id: 1,
    }),
    PlaylistFolderItem::Folder(PlaylistFolder {
      name: "Unnamed Folder".into(),
      generated_name: false,
      current_id: 0,
      target_id: 2,
    }),
  ];
  for language in [Language::En, Language::ZhHans, Language::ZhHant] {
    app.user_config.behavior.language = language;
    app.refresh_language();
    let text = buffer_text(&render_area(&app, 60, 15, library::draw_playlist_block));
    assert!(
      text.contains(app.strings().status.infra.unnamed_folder),
      "{text}"
    );
    assert!(text.contains("Unnamed Folder"), "{text}");
  }
}
