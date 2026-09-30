//! Music.app remote protocol. Parsing and ownership data have no Apple dependency.
//! The only process/Apple Event boundary is the opt-in macOS `macos` module.
#![cfg_attr(
  not(all(feature = "apple-music", target_os = "macos")),
  allow(dead_code)
)]

#[cfg(any(test, all(feature = "apple-music", target_os = "macos")))]
pub(crate) mod dispatch;
#[cfg(all(feature = "apple-music", target_os = "macos"))]
mod macos;
#[cfg(any(test, all(feature = "apple-music", target_os = "macos")))]
mod process;

use anyhow::{bail, ensure, Context, Result};
use serde::Deserialize;
use std::time::Instant;

use crate::core::plugin_api::{PlaylistInfo, TrackInfo};

pub(crate) const LIBRARY_URI: &str = "applemusic:library";
pub(crate) const PAGE_SIZE: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Browse {
  Playlists,
  Tracks(String),
  Search(String),
}

/// Persistent IDs are library-local 64-bit hexadecimal identifiers. Keep them
/// as strings: JavaScript numbers would round them above 2^53.
pub(crate) fn persistent_id(value: &str) -> Result<String> {
  ensure!(
    value.len() == 16 && value.bytes().all(|c| c.is_ascii_hexdigit()),
    "Invalid Apple Music persistent ID"
  );
  Ok(value.to_ascii_uppercase())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MusicUri {
  Track(String),
  Playlist(String),
  Library,
}

pub(crate) fn parse_uri(uri: &str) -> Result<MusicUri> {
  let value = uri
    .strip_prefix("applemusic:")
    .context("Not an Apple Music URI")?;
  if value == "library" {
    Ok(MusicUri::Library)
  } else if let Some(id) = value.strip_prefix("playlist:") {
    Ok(MusicUri::Playlist(persistent_id(id)?))
  } else {
    Ok(MusicUri::Track(persistent_id(value)?))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Command {
  Snapshot,
  Browse(Browse, usize),
  Play {
    container: MusicUri,
    track: Option<String>,
    offset: usize,
  },
  Resume,
  Pause,
  Next,
  Previous,
  Seek(u32),
  Volume(u8),
}

impl Command {
  /// All caller data is argv data, never source code or a shell command.
  pub(crate) fn arguments(&self) -> Result<Vec<String>> {
    let args: Vec<String> = match self {
      Self::Snapshot => vec!["snapshot".into()],
      Self::Resume => vec!["resume".into()],
      Self::Pause => vec!["pause".into()],
      Self::Next => vec!["next".into()],
      Self::Previous => vec!["previous".into()],
      Self::Seek(ms) => vec!["seek".into(), ms.to_string()],
      Self::Volume(volume) => vec!["volume".into(), volume.min(&100).to_string()],
      Self::Browse(Browse::Playlists, offset) => vec!["playlists".into(), offset.to_string()],
      Self::Browse(Browse::Tracks(uri), offset) => {
        let target = match parse_uri(uri)? {
          MusicUri::Playlist(id) => id,
          MusicUri::Library => "library".into(),
          MusicUri::Track(_) => bail!("Expected an Apple Music playlist"),
        };
        vec!["tracks".into(), target, offset.to_string()]
      }
      Self::Browse(Browse::Search(query), offset) => {
        ensure!(
          !query.contains('\0') && query.len() <= 4096,
          "Apple Music search is too long or contains NUL"
        );
        vec!["search".into(), query.clone(), offset.to_string()]
      }
      Self::Play {
        container,
        track,
        offset,
      } => {
        let (kind, id) = match container {
          MusicUri::Track(id) => ("track", persistent_id(id)?),
          MusicUri::Playlist(id) => ("playlist", persistent_id(id)?),
          MusicUri::Library => ("playlist", "library".into()),
        };
        vec![
          "play".into(),
          kind.into(),
          id,
          track
            .as_deref()
            .map(persistent_id)
            .transpose()?
            .unwrap_or_default(),
          offset.to_string(),
        ]
      }
    };
    Ok(args)
  }

  pub(crate) fn may_launch(&self) -> bool {
    !matches!(self, Self::Snapshot | Self::Pause)
  }
}

#[derive(Debug, Deserialize)]
struct WireTrack {
  id: String,
  name: String,
  #[serde(default)]
  artist: String,
  #[serde(default)]
  album: String,
  duration: f64,
}

fn milliseconds(seconds: f64) -> Result<u32> {
  ensure!(
    seconds.is_finite() && seconds >= 0.0 && seconds <= f64::from(u32::MAX) / 1000.0,
    "Invalid Apple Music time"
  );
  Ok((seconds * 1000.0).round() as u32)
}

impl WireTrack {
  fn into_track(self) -> Result<TrackInfo> {
    Ok(TrackInfo {
      uri: Some(format!("applemusic:{}", persistent_id(&self.id)?)),
      name: self.name,
      artists: if self.artist.is_empty() {
        vec![]
      } else {
        vec![self.artist]
      },
      album: self.album,
      duration_ms: milliseconds(self.duration)? as u64,
      id: None,
      album_id: None,
      artist_refs: vec![],
      is_playable: true,
      is_local: false,
      track_number: 0,
      explicit: false,
      image_url: None,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Snapshot {
  pub running: bool,
  pub playing: bool,
  pub track: Option<TrackInfo>,
  pub position_ms: u32,
  pub volume: u8,
}

pub(crate) fn parse_snapshot(json: &str) -> Result<Snapshot> {
  #[derive(Deserialize)]
  struct Wire {
    running: bool,
    playing: bool,
    track: Option<WireTrack>,
    position: f64,
    volume: u8,
  }
  let wire: Wire = serde_json::from_str(json).context("Invalid Music response")?;
  ensure!(wire.volume <= 100, "Invalid Music volume");
  ensure!(
    wire.running || (!wire.playing && wire.track.is_none()),
    "Inconsistent Music state"
  );
  let track = wire.track.map(WireTrack::into_track).transpose()?;
  let position_ms =
    milliseconds(wire.position)?.min(track.as_ref().map_or(0, |t| t.duration_ms as u32));
  Ok(Snapshot {
    running: wire.running,
    playing: wire.playing,
    track,
    position_ms,
    volume: wire.volume,
  })
}

#[derive(Debug)]
pub(crate) struct Page<T> {
  pub items: Vec<T>,
  pub offset: usize,
  pub total: usize,
}

#[derive(Deserialize)]
struct WirePage<T> {
  items: Vec<T>,
  offset: usize,
  total: usize,
}

fn validate_page<T>(page: &WirePage<T>) -> Result<()> {
  ensure!(
    page.items.len() <= PAGE_SIZE
      && page.offset <= page.total
      && page.items.len() <= page.total - page.offset,
    "Invalid Music page bounds"
  );
  ensure!(
    !page.items.is_empty() || page.offset == page.total,
    "Music returned an empty incomplete page"
  );
  Ok(())
}

pub(crate) fn parse_tracks(json: &str) -> Result<Page<TrackInfo>> {
  let wire: WirePage<WireTrack> = serde_json::from_str(json).context("Invalid Music tracks")?;
  validate_page(&wire)?;
  Ok(Page {
    items: wire
      .items
      .into_iter()
      .map(WireTrack::into_track)
      .collect::<Result<_>>()?,
    offset: wire.offset,
    total: wire.total,
  })
}

pub(crate) fn parse_playlists(json: &str) -> Result<Page<PlaylistInfo>> {
  #[derive(Deserialize)]
  struct Playlist {
    id: String,
    name: String,
  }
  let wire: WirePage<Playlist> = serde_json::from_str(json).context("Invalid Music playlists")?;
  validate_page(&wire)?;
  let items = wire
    .items
    .into_iter()
    .map(|p| {
      Ok(PlaylistInfo {
        uri: format!("applemusic:playlist:{}", persistent_id(&p.id)?),
        name: p.name,
        owner: "Music".into(),
        track_count: 0,
        id: None,
        owner_id: None,
        collaborative: false,
        public: None,
        image_url: None,
      })
    })
    .collect::<Result<_>>()?;
  Ok(Page {
    items,
    offset: wire.offset,
    total: wire.total,
  })
}

#[derive(Default)]
pub(crate) struct RemoteState {
  pub claimed: bool,
  pub switching: bool,
  pub desired_playing: bool,
  pub generation: u64,
  pub snapshot: Option<Snapshot>,
  pub observed_at: Option<Instant>,
  pub browse_generation: u64,
  pub browse: Option<Browse>,
  pub tracks: Vec<TrackInfo>,
  pub playlists: Vec<PlaylistInfo>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn apple_music_ids_never_become_code_or_imprecise_numbers() {
    assert_eq!(
      parse_uri("applemusic:ffffffffffffffff").unwrap(),
      MusicUri::Track("FFFFFFFFFFFFFFFF".into())
    );
    for bad in [
      "applemusic:",
      "applemusic:1",
      "applemusic:playlist:bad",
      "applemusic:1234567890ABCDEF;quit",
      "spotify:track:x",
    ] {
      assert!(parse_uri(bad).is_err());
    }
    let query = "\"; Application('Finder').activate(); //\n歌\\曲";
    assert_eq!(
      Command::Browse(Browse::Search(query.into()), 0)
        .arguments()
        .unwrap(),
      vec!["search", query, "0"]
    );
  }

  #[test]
  fn apple_music_json_round_trips_quotes_newlines_and_unicode() {
    let json = serde_json::json!({"running":true,"playing":true,"volume":42,"position":1.25,
      "track":{"id":"0123456789ABCDEF","name":"A\t\"B\"\n歌","artist":"合作者\\","album":"Album","duration":2.5}}).to_string();
    let snapshot = parse_snapshot(&json).unwrap();
    assert_eq!(snapshot.position_ms, 1250);
    let track = snapshot.track.unwrap();
    assert_eq!(track.name, "A\t\"B\"\n歌");
    assert_eq!(track.duration_ms, 2500);
    assert_eq!(track.id, None);
  }

  #[test]
  fn apple_music_rejects_malformed_times_pages_and_partial_json() {
    for time in [-1.0, f64::NAN, f64::INFINITY, 1e20] {
      assert!(milliseconds(time).is_err());
    }
    assert!(parse_snapshot("{\"running\":true}").is_err());
    assert!(parse_tracks(r#"{"items":[],"offset":0,"total":1}"#).is_err());
    assert!(parse_playlists(r#"{"items":[],"offset":2,"total":1}"#).is_err());
    assert!(parse_tracks(r#"{"items":[],"offset":0,"total":0}"#).is_ok());
    assert!(!Command::Pause.may_launch());
    assert!(!Command::Snapshot.may_launch());
  }
}
