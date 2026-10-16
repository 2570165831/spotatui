//! Visible DJ tool summaries; protocol and model-facing tool text stays unchanged.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub struct ToolStrings {
  pub skipped: &'static str,
  pub vibe_set: &'static str,
  pub vibe_cleared: &'static str,
  pub vibe_on: &'static str,
  pub vibe_off: &'static str,
  pub vibe_unavailable: &'static str,
  pub creating: &'static str,
  pub adding: &'static str,
  pub removing: &'static str,
  pub following: &'static str,
  pub unfollowing: &'static str,
  pub transfer: &'static str,
  pub liking: &'static str,
  pub unliking: &'static str,
  pub search_failed: &'static str,
  pub no_tracks: &'static str,
  pub found_track: &'static str,
  pub owned: &'static str,
  pub new_track: &'static str,
  pub queued: &'static str,
  pub queue_empty: &'static str,
  pub nothing_playing: &'static str,
  pub now_playing: &'static str,
  pub playing: &'static str,
  pub paused: &'static str,
  pub sent: &'static str,
  pub play_started: &'static str,
  pub missing_feature: &'static str,
  pub missing_track: &'static str,
  pub history_failed: &'static str,
  pub history_task_failed: &'static str,
  pub history_sparse: &'static str,
  pub history_summary: &'static str,
  pub catalogue_unavailable: &'static str,
  pub session_unavailable: &'static str,
  pub uneditable: &'static str,
  pub queue_filter_failed: &'static str,
  pub nothing_queued: &'static str,
  pub unknown_tool: &'static str,
  pub invalid_arguments: &'static str,
  pub shutting_down: &'static str,
  pub dropped: &'static str,
  pub timed_out: &'static str,
  pub brief_artists: &'static str,
  pub brief_tracks: &'static str,
  pub brief_albums: &'static str,
  pub brief_recent: &'static str,
  pub brief_now: &'static str,
  pub brief_vibe: &'static str,
  pub period_week: &'static str,
  pub period_month: &'static str,
  pub period_this_month: &'static str,
  pub period_year: &'static str,
  pub period_all: &'static str,
  pub ownership_incomplete: &'static str,
  pub queue_not_found: &'static str,
  pub queue_owned: &'static str,
  pub queue_duplicate: &'static str,
  pub queue_filter_spotify_only: &'static str,
  pub none_found: &'static str,
  pub none_owned: &'static str,
  pub none_duplicate: &'static str,
  pub none_new: &'static str,
}
pub const EN: ToolStrings = ToolStrings {
  brief_artists: "Top artists",
  brief_tracks: "Top tracks",
  brief_albums: "Top albums",
  brief_recent: "Recently played (do not repeat these)",
  brief_now: "\nCurrently playing: {name}\n",
  brief_vibe: "\nThe listener asked for: {name}\n",
  period_week: "the last 7 days",
  period_month: "the last 30 days",
  period_this_month: "this month",
  period_year: "this year",
  period_all: "all time",
  ownership_incomplete: "\n\nNote: the playlist index is still being built, so [owned] reflects Liked Songs only for this search. Search again for the complete answer.",
  queue_not_found: "\n\nNot found (skipped): {names}",
  queue_owned: "\nAlready in your library (skipped by exclude_owned): {names}",
  queue_duplicate: "\nAlready queued, playing, or recently played (skipped): {names}",
  queue_filter_spotify_only: "\nexclude_owned only checks Spotify tracks, so it skipped nothing here.",
  none_found: "Could not find: {names}",
  none_owned: "Already in your library: {names}",
  none_duplicate: "Already queued, playing, or recently played: {names}",
  none_new: "Every track was already queued or playing.",
  skipped: "Skipped to the next track",
  vibe_set: "DJ vibe set to: {vibe}. {effect}",
  vibe_cleared: "DJ vibe cleared",
  vibe_on: "The built-in DJ will follow it from its next refill.",
  vibe_off: "The built-in DJ's auto-queue is off, so nothing will act on it until the user turns it on; queue tracks yourself in the meantime. get_listening_history returns it either way.",
  vibe_unavailable: "This build has no built-in auto-queue DJ, so nothing will act on it automatically. It is stored and returned by get_listening_history, so you can follow it yourself when you queue.",
  creating: "Creating the playlist \"{name}\" with {count} track(s). It is in list_playlists once Spotify has made it.",
  adding: "Adding {count} track(s) to {playlist}",
  removing: "Removing {uri} at position {position} from {playlist}, if that is the track there",
  following: "Following spotify:artist:{artist}",
  unfollowing: "Unfollowing spotify:artist:{artist}",
  transfer: "Moving playback to {name}",
  liking: "Liking {changed} track(s); {unchanged} already liked",
  unliking: "Unliking {changed} track(s); {unchanged} already not liked",
  search_failed: "Search failed: {error}",
  no_tracks: "No tracks found for \"{query}\"",
  found_track: "{name} — {artists} [{uri}] [{owned}]",
  owned: "owned",
  new_track: "new",
  queued: "Queued {count} track(s):",
  queue_empty: "The queue is empty",
  nothing_playing: "Nothing is playing ({count} track(s) queued)",
  now_playing: "{state}: {label} ({count} track(s) queued)",
  playing: "playing",
  paused: "paused",
  sent: "Sent {uri} to the player",
  play_started: "Playing {label}",
  missing_feature: "Nothing is playing that: this spotatui was built without the `{feature}` feature, so it has no source that can play {uri}. Playback was left alone.",
  missing_track: "Nothing is playing that: {uri} is not in the catalogue, so playback was left alone. Use search_tracks to get a URI that exists.",
  history_failed: "Could not read listening history: {error}",
  history_task_failed: "Listening history task failed: {error}",
  history_sparse: "Very little listening history so far ({count} qualifying plays in {period}). Ask the user what they feel like instead of inferring it.",
  history_summary: "Listening history ({period}, {count} qualifying plays):",
  catalogue_unavailable: "spotatui has no Spotify session, so the catalogue is unavailable. Log in from the spotatui UI, then try again.",
  session_unavailable: "spotatui has no Spotify session. Log in from the spotatui UI, then try again.",
  uneditable: "\"{name}\" is not a playlist the user owns or collaborates on, so it cannot be edited",
  queue_filter_failed: "Could not read your playlists, so exclude_owned cannot be honoured and nothing was queued. Try again in a moment, or call queue_tracks without exclude_owned to queue these anyway.",
  nothing_queued: "Nothing was queued.\n{detail}",
  unknown_tool: "There is no tool called {name}. Use one of the tools listed above.",
  invalid_arguments: "Invalid arguments: {detail}",
  shutting_down: "spotatui is shutting down; the request was not run",
  dropped: "spotatui dropped the request before answering",
  timed_out: "spotatui did not answer in time; it may be busy. Try again in a moment.",
};
// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANS: ToolStrings = ToolStrings { ..EN };
// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: ToolStrings = ToolStrings { ..EN };
#[cfg(any(feature = "dj-core", test))]
impl ToolStrings {
  pub fn brief_now(&self, name: &str) -> String {
    super::format_template(self.brief_now, &[("name", name)])
  }

  pub fn brief_vibe(&self, name: &str) -> String {
    super::format_template(self.brief_vibe, &[("name", name)])
  }

  pub fn queue_not_found(&self, names: &str) -> String {
    super::format_template(self.queue_not_found, &[("names", names)])
  }

  pub fn queue_owned(&self, names: &str) -> String {
    super::format_template(self.queue_owned, &[("names", names)])
  }

  pub fn queue_duplicate(&self, names: &str) -> String {
    super::format_template(self.queue_duplicate, &[("names", names)])
  }

  pub fn none_found(&self, names: &str) -> String {
    super::format_template(self.none_found, &[("names", names)])
  }

  pub fn none_owned(&self, names: &str) -> String {
    super::format_template(self.none_owned, &[("names", names)])
  }

  pub fn none_duplicate(&self, names: &str) -> String {
    super::format_template(self.none_duplicate, &[("names", names)])
  }

  pub fn vibe_set(&self, vibe: &str, effect: &str) -> String {
    super::format_template(self.vibe_set, &[("vibe", vibe), ("effect", effect)])
  }
  pub fn creating(&self, name: &str, count: usize) -> String {
    super::format_template(
      self.creating,
      &[("name", name), ("count", &count.to_string())],
    )
  }
  pub fn adding(&self, count: usize, playlist: &str) -> String {
    super::format_template(
      self.adding,
      &[("count", &count.to_string()), ("playlist", playlist)],
    )
  }
  pub fn removing(&self, uri: &str, position: usize, playlist: &str) -> String {
    super::format_template(
      self.removing,
      &[
        ("uri", uri),
        ("position", &position.to_string()),
        ("playlist", playlist),
      ],
    )
  }
  pub fn following(&self, artist: &str) -> String {
    super::format_template(self.following, &[("artist", artist)])
  }
  pub fn unfollowing(&self, artist: &str) -> String {
    super::format_template(self.unfollowing, &[("artist", artist)])
  }
  pub fn transfer(&self, name: &str) -> String {
    super::format_template(self.transfer, &[("name", name)])
  }
  pub fn liking(&self, changed: usize, unchanged: usize) -> String {
    super::format_template(
      self.liking,
      &[
        ("changed", &changed.to_string()),
        ("unchanged", &unchanged.to_string()),
      ],
    )
  }
  pub fn unliking(&self, changed: usize, unchanged: usize) -> String {
    super::format_template(
      self.unliking,
      &[
        ("changed", &changed.to_string()),
        ("unchanged", &unchanged.to_string()),
      ],
    )
  }
  pub fn search_failed(&self, error: &str) -> String {
    super::format_template(self.search_failed, &[("error", error)])
  }
  pub fn no_tracks(&self, query: &str) -> String {
    super::format_template(self.no_tracks, &[("query", query)])
  }
  pub fn found_track(&self, name: &str, artists: &str, uri: &str, owned: &str) -> String {
    super::format_template(
      self.found_track,
      &[
        ("name", name),
        ("artists", artists),
        ("uri", uri),
        ("owned", owned),
      ],
    )
  }
  pub fn queued(&self, count: usize) -> String {
    super::format_template(self.queued, &[("count", &count.to_string())])
  }
  pub fn nothing_playing(&self, count: usize) -> String {
    super::format_template(self.nothing_playing, &[("count", &count.to_string())])
  }
  pub fn now_playing(&self, state: &str, label: &str, count: usize) -> String {
    super::format_template(
      self.now_playing,
      &[
        ("state", state),
        ("label", label),
        ("count", &count.to_string()),
      ],
    )
  }
  pub fn sent(&self, uri: &str) -> String {
    super::format_template(self.sent, &[("uri", uri)])
  }
  pub fn play_started(&self, label: &str) -> String {
    super::format_template(self.play_started, &[("label", label)])
  }
  pub fn missing_feature(&self, feature: &str, uri: &str) -> String {
    super::format_template(self.missing_feature, &[("feature", feature), ("uri", uri)])
  }
  pub fn missing_track(&self, uri: &str) -> String {
    super::format_template(self.missing_track, &[("uri", uri)])
  }
  pub fn history_failed(&self, error: &str) -> String {
    super::format_template(self.history_failed, &[("error", error)])
  }
  pub fn history_task_failed(&self, error: &str) -> String {
    super::format_template(self.history_task_failed, &[("error", error)])
  }
  pub fn history_sparse(&self, count: usize, period: &str) -> String {
    super::format_template(
      self.history_sparse,
      &[("count", &count.to_string()), ("period", period)],
    )
  }
  pub fn history_summary(&self, period: &str, count: usize) -> String {
    super::format_template(
      self.history_summary,
      &[("period", period), ("count", &count.to_string())],
    )
  }
  pub fn uneditable(&self, name: &str) -> String {
    super::format_template(self.uneditable, &[("name", name)])
  }
  pub fn nothing_queued(&self, detail: &str) -> String {
    super::format_template(self.nothing_queued, &[("detail", detail)])
  }
  #[cfg(any(feature = "ai-dj", test))]
  pub fn unknown_tool(&self, name: &str) -> String {
    super::format_template(self.unknown_tool, &[("name", name)])
  }
  #[cfg(any(feature = "ai-dj", test))]
  pub fn invalid_arguments(&self, detail: &str) -> String {
    super::format_template(self.invalid_arguments, &[("detail", detail)])
  }
}
