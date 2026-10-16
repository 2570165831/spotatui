use crate::core::app::{ActiveBlock, App, FriendAddMode, FriendEntry, FriendFilter};
use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::{Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
  Frame,
};

use super::util::{get_color, hint_span, truncate_text};
use crate::tui::theme::EmphasisExt;

pub fn draw_friends(f: &mut Frame<'_>, app: &App, layout_chunk: Rect) {
  let current_route = app.get_current_route();
  let highlight_state = (
    current_route.active_block == ActiveBlock::Friends,
    current_route.hovered_block == ActiveBlock::Friends,
  );

  let outer_block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(get_color(highlight_state, app.user_config.theme))
    .title(Span::styled(
      app.strings().screens.friends_title,
      get_color(highlight_state, app.user_config.theme)
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    ));

  let inner = outer_block.inner(layout_chunk);
  f.render_widget(outer_block, layout_chunk);

  // No sync token → show account-required prompt
  if app.user_config.behavior.sync_token.is_none() {
    draw_no_token_prompt(f, app, inner);
    return;
  }

  // Split inner area: friend-code card | tabs+list | help bar
  let [top_area, list_area, help_area] = inner.layout(&Layout::vertical([
    Constraint::Length(3),
    Constraint::Min(1),
    Constraint::Length(1),
  ]));

  draw_friend_code_card(f, app, top_area);
  draw_friends_body(f, app, list_area);
  draw_help_bar(f, app, help_area);

  // Add-friend overlay rendered on top
  if app.view.friend_add_dialog_visible {
    draw_add_friend_dialog(f, app, layout_chunk);
  }
}

// ── No-token prompt ───────────────────────────────────────────────────────────

fn draw_no_token_prompt(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  let lines = vec![
    Line::default(),
    Line::default(),
    Line::from(Span::styled(
      app.strings().screens.account_required,
      Style::default()
        .fg(theme.banner.into())
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    )),
    Line::default(),
    Line::from(Span::styled(
      app.strings().screens.friends_account_line1,
      Style::default().fg(theme.text.into()),
    )),
    Line::from(Span::styled(
      app.strings().screens.friends_account_line2,
      Style::default().fg(theme.text.into()),
    )),
    Line::default(),
    Line::from(vec![
      Span::styled(
        app.strings().screens.sign_up_at,
        Style::default().fg(theme.inactive.into()),
      ),
      Span::styled(
        "https://spotatui.com",
        Style::default()
          .fg(theme.hint.into())
          .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
      ),
    ]),
    Line::default(),
    Line::from(Span::styled(
      app.strings().screens.sync_token_line1,
      Style::default().fg(theme.inactive.into()),
    )),
    Line::from(Span::styled(
      app.strings().screens.sync_token_line2,
      Style::default().fg(theme.inactive.into()),
    )),
    Line::default(),
    Line::from(Span::styled(
      app.strings().screens.esc_back,
      Style::default().fg(theme.hint.into()),
    )),
  ];

  let para = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
  f.render_widget(para, area);
}

// ── Friend code card ──────────────────────────────────────────────────────────

fn draw_friend_code_card(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  let code_text = app
    .friend_code
    .as_deref()
    .unwrap_or(app.strings().screens.loading);

  let hint = app.strings().screens.copy_code_short;

  let line = Line::from(vec![
    Span::styled(
      app.strings().screens.your_code,
      Style::default()
        .fg(theme.inactive.into())
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    ),
    Span::styled(
      code_text,
      Style::default()
        .fg(theme.hint.into())
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    ),
    Span::raw("  "),
    Span::styled(hint, Style::default().fg(theme.inactive.into())),
  ]);

  let card = Paragraph::new(line).block(
    Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(theme.active.into())),
  );

  f.render_widget(card, area);
}

// ── Main body: filter tabs + friend list ──────────────────────────────────────

fn draw_friends_body(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  // Split into a thin tab/control row and the list below
  let [controls_area, list_area] = area.layout(&Layout::vertical([
    Constraint::Length(1),
    Constraint::Min(1),
  ]));

  // Controls row: [All (n)] [Online (n)]  ... search hint ... [+ Add Friend]
  draw_filter_tabs(f, app, controls_area);

  // The actual friends list
  let filtered = filtered_friends(app);

  if filtered.is_empty() {
    let msg = if !app.view.friend_search_input.is_empty() {
      app.strings().screens.friends_no_match
    } else if app.friends_loading {
      app.strings().screens.friends_loading
    } else if app.friends.is_empty() {
      app.strings().screens.friends_empty
    } else {
      app.strings().screens.friends_no_online
    };
    let para = Paragraph::new(Span::styled(
      msg,
      Style::default().fg(theme.inactive.into()),
    ))
    .alignment(ratatui::layout::Alignment::Center);
    // Center vertically a bit
    let [_, center, _] = list_area.layout(&Layout::vertical([
      Constraint::Percentage(30),
      Constraint::Length(1),
      Constraint::Min(1),
    ]));
    f.render_widget(para, center);
    return;
  }

  draw_friend_list(f, app, list_area, &filtered);
}

fn draw_filter_tabs(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;
  let online_count = app.friends.iter().filter(|f| f.is_online).count();
  let all_count = app.friends.len();

  let all_style = if app.view.friend_filter == FriendFilter::All {
    Style::default()
      .fg(theme.selected.into())
      .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD))
  } else {
    Style::default().fg(theme.inactive.into())
  };
  let online_style = if app.view.friend_filter == FriendFilter::Online {
    Style::default()
      .fg(theme.selected.into())
      .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD))
  } else {
    Style::default().fg(theme.inactive.into())
  };

  // Build search preview (inline: any unbound key types into the filter)
  let search_str: String = app.view.friend_search_input.iter().collect();
  let search_hint = if search_str.is_empty() {
    app.strings().screens.search_placeholder.to_string()
  } else {
    format!("/{}/", search_str)
  };

  let line = Line::from(vec![
    Span::styled(app.strings().screens.friends_all(all_count), all_style),
    Span::styled(" │ ", Style::default().fg(theme.inactive.into())),
    Span::styled(
      app.strings().screens.friends_online(online_count),
      online_style,
    ),
    Span::styled("    ", Style::default()),
    Span::styled(search_hint, Style::default().fg(theme.inactive.into())),
    Span::styled("    ", Style::default()),
    Span::styled(
      app.strings().screens.add_friend_short,
      Style::default().fg(theme.active.into()),
    ),
    Span::raw(" "),
  ]);

  let para = Paragraph::new(line);
  f.render_widget(para, area);
}

fn draw_friend_list(f: &mut Frame<'_>, app: &App, area: Rect, friends: &[&FriendEntry]) {
  let theme = app.user_config.theme;

  let items: Vec<ListItem> = friends
    .iter()
    .enumerate()
    .map(|(i, friend)| {
      let is_selected = i == app.view.friend_selected_index;

      // Online indicator
      let online_span = if friend.is_online {
        Span::styled("● ", Style::default().fg(theme.active.into()))
      } else {
        Span::styled("○ ", Style::default().fg(theme.inactive.into()))
      };

      // Name
      let name_style = if is_selected {
        Style::default()
          .fg(theme.selected.into())
          .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD))
      } else {
        Style::default().fg(theme.text.into())
      };
      let name_span = Span::styled(
        super::util::pad_to_width(&truncate_text(&friend.name, 20), 20),
        name_style,
      );

      // Now-playing or status
      let np_spans = if let Some(np) = &friend.now_playing {
        vec![
          Span::styled("▶ ", Style::default().fg(theme.active.into())),
          Span::styled(
            truncate_text(&np.title, 28),
            Style::default().fg(theme.hint.into()),
          ),
          Span::styled(
            format!(" — {}", truncate_text(&np.artists, 20)),
            Style::default().fg(theme.inactive.into()),
          ),
        ]
      } else if friend.is_online {
        vec![Span::styled(
          app.strings().screens.idle,
          Style::default().fg(theme.inactive.into()),
        )]
      } else {
        vec![Span::styled(
          app.strings().screens.offline,
          Style::default().fg(theme.inactive.into()),
        )]
      };

      let mut spans = vec![online_span, name_span, Span::raw("  ")];
      spans.extend(np_spans);

      ListItem::new(Line::from(spans))
    })
    .collect();

  let mut state = ListState::default();
  state.select(Some(
    app
      .view
      .friend_selected_index
      .min(friends.len().saturating_sub(1)),
  ));

  let list = List::new(items)
    .highlight_style(
      Style::default()
        .fg(theme.selected.into())
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    )
    .highlight_symbol("▶ ");

  f.render_stateful_widget(list, area, &mut state);
}

// ── Help bar ──────────────────────────────────────────────────────────────────

fn draw_help_bar(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  let line = Line::from(vec![
    hint_span("↑/↓", theme),
    Span::styled(
      app.strings().screens.navigate_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("c", theme),
    Span::styled(
      app.strings().screens.copy_code_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("a", theme),
    Span::styled(
      app.strings().screens.add_friend_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("u", theme),
    Span::styled(
      app.strings().screens.unfollow_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("Tab", theme),
    Span::styled(
      app.strings().screens.filter_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    Span::styled(
      app.strings().screens.type_search_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("Esc", theme),
    Span::styled(
      app.strings().screens.back_hint,
      Style::default().fg(theme.inactive.into()),
    ),
  ]);

  f.render_widget(Paragraph::new(line), area);
}

// ── Add-Friend dialog overlay ─────────────────────────────────────────────────

fn draw_add_friend_dialog(f: &mut Frame<'_>, app: &App, parent: Rect) {
  let theme = app.user_config.theme;

  // Center a 50×18 dialog box
  let dialog_width = 52u16.min(parent.width.saturating_sub(4));
  let dialog_height = 14u16.min(parent.height.saturating_sub(4));
  let x = parent.x + (parent.width.saturating_sub(dialog_width)) / 2;
  let y = parent.y + (parent.height.saturating_sub(dialog_height)) / 2;
  let dialog_area = Rect::new(x, y, dialog_width, dialog_height);

  f.render_widget(Clear, dialog_area);

  let block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::default().fg(theme.active.into()))
    .title(Span::styled(
      app.strings().screens.add_friend_title,
      Style::default()
        .fg(theme.active.into())
        .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
    ));

  let inner = block.inner(dialog_area);
  f.render_widget(block, dialog_area);

  // Mode tabs
  let [tabs_area, content_area, help_area] = inner.layout(&Layout::vertical([
    Constraint::Length(1),
    Constraint::Min(1),
    Constraint::Length(1),
  ]));

  // Tab row
  let code_style = if app.view.friend_add_mode == FriendAddMode::Code {
    Style::default().fg(theme.selected.into()).add_modifier(
      app
        .user_config
        .behavior
        .emphasis(Modifier::BOLD | Modifier::UNDERLINED),
    )
  } else {
    Style::default().fg(theme.inactive.into())
  };
  let search_style = if app.view.friend_add_mode == FriendAddMode::Search {
    Style::default().fg(theme.selected.into()).add_modifier(
      app
        .user_config
        .behavior
        .emphasis(Modifier::BOLD | Modifier::UNDERLINED),
    )
  } else {
    Style::default().fg(theme.inactive.into())
  };
  let tabs_line = Line::from(vec![
    Span::styled(app.strings().screens.friend_code_tab, code_style),
    Span::styled(" │ ", Style::default().fg(theme.inactive.into())),
    Span::styled(app.strings().screens.friend_name_tab, search_style),
  ]);
  f.render_widget(Paragraph::new(tabs_line), tabs_area);

  match app.view.friend_add_mode {
    FriendAddMode::Code => draw_add_by_code(f, app, content_area),
    FriendAddMode::Search => draw_add_by_search(f, app, content_area),
  }

  // Footer hints
  let footer_line = Line::from(vec![
    hint_span("Tab", theme),
    Span::styled(
      app.strings().screens.switch_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("Enter", theme),
    Span::styled(
      app.strings().screens.add_hint,
      Style::default().fg(theme.inactive.into()),
    ),
    hint_span("Esc", theme),
    Span::styled(
      app.strings().screens.cancel_hint,
      Style::default().fg(theme.inactive.into()),
    ),
  ]);
  f.render_widget(Paragraph::new(footer_line), help_area);
}

fn draw_add_by_code(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  let input: String = app.view.friend_add_input.iter().collect();
  let display = if input.is_empty() {
    app.strings().screens.friend_code_placeholder.to_string()
  } else {
    input.clone()
  };
  let style = if input.is_empty() {
    Style::default().fg(theme.inactive.into())
  } else {
    Style::default()
      .fg(theme.hint.into())
      .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD))
  };

  let [_, input_row, hint_row, _] = area.layout(&Layout::vertical([
    Constraint::Length(1),
    Constraint::Length(3),
    Constraint::Length(1),
    Constraint::Min(0),
  ]));

  let input_widget = Paragraph::new(Span::styled(display, style))
    .alignment(ratatui::layout::Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.active.into())),
    );
  f.render_widget(input_widget, input_row);

  let hint = Paragraph::new(Span::styled(
    app.strings().screens.friend_code_enter,
    Style::default().fg(theme.inactive.into()),
  ))
  .alignment(ratatui::layout::Alignment::Center);
  f.render_widget(hint, hint_row);
}

fn draw_add_by_search(f: &mut Frame<'_>, app: &App, area: Rect) {
  let theme = app.user_config.theme;

  let [input_row, results_area] = area.layout(&Layout::vertical([
    Constraint::Length(3),
    Constraint::Min(1),
  ]));

  // Search input field
  let search_str: String = app.view.friend_user_search_input.iter().collect();
  let search_display = if search_str.is_empty() {
    app.strings().screens.friend_search_placeholder.to_string()
  } else {
    search_str
  };
  let search_style = if app.view.friend_user_search_input.is_empty() {
    Style::default().fg(theme.inactive.into())
  } else {
    Style::default().fg(theme.text.into())
  };
  let search_widget = Paragraph::new(Span::styled(search_display, search_style)).block(
    Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(theme.inactive.into())),
  );
  f.render_widget(search_widget, input_row);

  // Results list
  if app.friend_user_search_results.is_empty() {
    let msg = if app.view.friend_user_search_input.is_empty() {
      ""
    } else {
      app.strings().screens.users_empty
    };
    f.render_widget(
      Paragraph::new(Span::styled(
        msg,
        Style::default().fg(theme.inactive.into()),
      ))
      .alignment(ratatui::layout::Alignment::Center),
      results_area,
    );
    return;
  }

  let items: Vec<ListItem> = app
    .friend_user_search_results
    .iter()
    .enumerate()
    .map(|(i, r)| {
      let is_sel = i == app.view.friend_user_search_selected;
      let prefix = if is_sel { "▶ " } else { "  " };
      let style = if is_sel {
        Style::default()
          .fg(theme.selected.into())
          .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD))
      } else {
        Style::default().fg(theme.text.into())
      };
      let following_tag = if r.is_following {
        app.strings().screens.following_tag
      } else {
        ""
      };
      ListItem::new(Line::from(vec![
        Span::styled(prefix, Style::default().fg(theme.selected.into())),
        Span::styled(format!("{}{}", r.name, following_tag), style),
      ]))
    })
    .collect();

  let mut state = ListState::default();
  state.select(Some(app.view.friend_user_search_selected));
  let list = List::new(items).highlight_style(
    Style::default()
      .fg(theme.selected.into())
      .add_modifier(app.user_config.behavior.emphasis(Modifier::BOLD)),
  );
  f.render_stateful_widget(list, results_area, &mut state);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Return the subset of friends that pass the active filter and search query.
pub fn filtered_friends(app: &App) -> Vec<&FriendEntry> {
  let search_str: String = app.view.friend_search_input.iter().collect();
  let q = search_str.to_lowercase();

  app
    .friends
    .iter()
    .filter(|f| {
      let passes_filter = match app.view.friend_filter {
        FriendFilter::All => true,
        FriendFilter::Online => f.is_online,
      };
      let passes_search = q.is_empty() || f.name_lower.contains(&q);
      passes_filter && passes_search
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::i18n::Language;
  use ratatui::{backend::TestBackend, Terminal};
  use unicode_width::UnicodeWidthChar;

  #[test]
  fn friend_status_columns_align_for_ascii_and_chinese_names() {
    let friends: Vec<FriendEntry> = [
      "Alice",
      "中文",
      "Alice中文",
      "這個好友名稱超過二十個終端儲存格",
      "na\u{0301}me",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, name)| FriendEntry {
      id: index.to_string(),
      name: name.into(),
      name_lower: name.to_lowercase(),
      is_online: true,
      now_playing: None,
      listening_ms: 0,
      total_listens: 0,
    })
    .collect();
    let rows: Vec<&FriendEntry> = friends.iter().collect();

    for language in [Language::En, Language::ZhHans, Language::ZhHant] {
      let mut app = App::default();
      app.user_config.behavior.language = language;
      app.refresh_language();
      let mut terminal = Terminal::new(TestBackend::new(64, rows.len() as u16)).unwrap();
      terminal
        .draw(|frame| draw_friend_list(frame, &app, frame.area(), &rows))
        .unwrap();
      let buffer = terminal.backend().buffer();

      for (y, friend) in rows.iter().enumerate() {
        // Selection marker (2), online marker (2), name (20), gap (2).
        let mut x = 26;
        for character in app.strings().screens.idle.chars() {
          assert_eq!(
            buffer.cell((x, y as u16)).unwrap().symbol(),
            character.to_string(),
            "{language:?}: status column shifted for {:?}",
            friend.name,
          );
          x += character.width().unwrap_or(0) as u16;
        }
      }
    }
  }
}
