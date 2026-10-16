//! Sidebar labels and messages shared by frontends.

use super::format_template;

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub struct SidebarStrings {
  pub library: &'static str,
  pub discover: &'static str,
  pub recently_played: &'static str,
  pub friends: &'static str,
  pub stats: &'static str,
  pub playlist_sync: &'static str,
  pub liked_songs: &'static str,
  pub albums: &'static str,
  pub artists: &'static str,
  pub podcasts: &'static str,
  pub local_files: &'static str,
  pub ai_dj: &'static str,
  pub community_pin: &'static str,
  pub empty_local_template: &'static str,
  pub empty_apple_music_template: &'static str,
  pub empty_subsonic_template: &'static str,
  pub empty_qobuz_template: &'static str,
  pub empty_radio: &'static str,
  pub radio_stations: &'static str,
  pub new_playlist: &'static str,
  pub youtube_playlists: &'static str,
  pub unknown: &'static str,
  pub add_playlist: &'static str,
  pub playlists: &'static str,
  pub all_songs: &'static str,
  pub favorite_tracks: &'static str,
  pub spotify: &'static str,
  pub subsonic: &'static str,
  pub internet_radio: &'static str,
  pub youtube: &'static str,
  pub qobuz: &'static str,
  pub apple_music: &'static str,
}

pub const EN: SidebarStrings = SidebarStrings {
  library: "Library",
  discover: "Discover",
  recently_played: "Recently Played",
  friends: "Friends",
  stats: "Stats",
  playlist_sync: "Playlist sync",
  liked_songs: "Liked Songs",
  albums: "Albums",
  artists: "Artists",
  podcasts: "Podcasts",
  local_files: "Local Files",
  ai_dj: "AI DJ",
  community_pin: "📌 spotatui community",
  empty_local_template: "(no folders — set music dir, then press `{key}`)",
  empty_apple_music_template: "(press `{key}`, pick Apple Music to load your library)",
  empty_subsonic_template: "(no playlists — configure server, then press `{key}`)",
  empty_qobuz_template: "(not logged in — press `{key}`, pick Qobuz)",
  empty_radio: "(no saved stations — search to add)",
  radio_stations: "Radio Stations",
  new_playlist: "+ New Playlist",
  youtube_playlists: "YouTube Playlists",
  unknown: "Unknown",
  add_playlist: "+ Add Playlist",
  playlists: "Playlists",
  all_songs: "All songs",
  favorite_tracks: "Favorite tracks",
  spotify: "Spotify",
  subsonic: "Subsonic",
  internet_radio: "Internet Radio",
  youtube: "YouTube",
  qobuz: "Qobuz",
  apple_music: "Apple Music",
};

pub const ZH_HANS: SidebarStrings = SidebarStrings { ..EN };
pub const ZH_HANT: SidebarStrings = SidebarStrings { ..EN };

impl SidebarStrings {
  /// Resolve only reserved synthetic rows; a user playlist with the same name
  /// remains untouched. URI literals also work in builds without that source.
  pub fn playlist_label<'a>(
    &self,
    source: crate::core::source::Source,
    uri: &str,
    name: &'a str,
  ) -> &'a str {
    use crate::core::source::Source;
    match source {
      Source::AppleMusic if uri == "applemusic:library" => self.all_songs,
      Source::Qobuz if uri == "qobuz:favorites:tracks" => self.favorite_tracks,
      Source::Spotify
      | Source::Local
      | Source::Subsonic
      | Source::Radio
      | Source::YouTube
      | Source::Qobuz
      | Source::AppleMusic => name,
    }
  }

  pub fn source_label(&self, source: crate::core::source::Source) -> &'static str {
    use crate::core::source::Source;
    match source {
      Source::Spotify => self.spotify,
      Source::Local => self.local_files,
      Source::Subsonic => self.subsonic,
      Source::Radio => self.internet_radio,
      Source::YouTube => self.youtube,
      Source::Qobuz => self.qobuz,
      Source::AppleMusic => self.apple_music,
    }
  }

  pub fn empty_local(&self, key: &str) -> String {
    format_template(self.empty_local_template, &[("key", key)])
  }

  pub fn empty_apple_music(&self, key: &str) -> String {
    format_template(self.empty_apple_music_template, &[("key", key)])
  }

  pub fn empty_subsonic(&self, key: &str) -> String {
    format_template(self.empty_subsonic_template, &[("key", key)])
  }

  pub fn empty_qobuz(&self, key: &str) -> String {
    format_template(self.empty_qobuz_template, &[("key", key)])
  }
}
