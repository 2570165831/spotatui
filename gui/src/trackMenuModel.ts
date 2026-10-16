import { EN, type GuiStrings } from "./i18n";
import type { Action } from "./bindings/Action";
import type { PlaylistInfo } from "./bindings/PlaylistInfo";
import type { TrackInfo } from "./bindings/TrackInfo";
import { sourceOf } from "./format";
import type { View } from "./nav";

/** One entry of a track's menu: it sends an action, opens a view, copies text or opens a submenu. */
export type MenuEntry = { id: string } & (
  | { label: string; kind: "action"; action: Action }
  | { label: string; kind: "open"; view: View }
  | {
      label: string;
      kind: "copy";
      text: string;
      done: "copied_song_link" | "copied_album_link";
    }
  | {
      label: string;
      kind: "submenu";
      entries: MenuEntry[];
    }
);

/** What the list around a row adds to its menu. */
export interface RowContext {
  /** Removes this row from the playlist the list shows. */
  remove?: Action | null;
}

/** A Spotify track the heart and the Spotify-only entries can act on. */
export function spotifyTrack(track: TrackInfo): boolean {
  return sourceOf(track.uri) === "Spotify" && track.id !== null;
}

/** The menu of a track row, in groups the menu separates with a rule. */
export function trackMenu(
  track: TrackInfo,
  liked: boolean,
  playlists: PlaylistInfo[],
  context: RowContext = {},
  s: GuiStrings = EN,
): MenuEntry[][] {
  const uri = track.uri;
  const spotify = spotifyTrack(track);
  const song = spotify && (uri?.startsWith("spotify:track:") ?? false);
  const edit: MenuEntry[] = [];
  if (spotify && uri && playlists.length > 0)
    edit.push({
      id: "add-playlist",
      label: s.add_playlist,
      kind: "submenu",
      entries: playlists.map((playlist) => ({
        id: playlist.uri,
        label: playlist.name,
        kind: "action",
        action: { AddTrackToPlaylist: { playlist: playlist.uri, track: uri } },
      })),
    });
  if (context.remove)
    edit.push({
      id: "remove",
      label: s.remove_from_playlist,
      kind: "action",
      action: context.remove,
    });
  if (spotify && uri)
    edit.push({
      id: "save",
      label: liked ? s.remove_your_liked : s.save_your_liked,
      kind: "action",
      action: { ToggleSaveTrack: uri },
    });
  const source = sourceOf(uri);
  if (uri && source !== "Radio" && source !== "AppleMusic")
    edit.push({
      id: "queue",
      label: s.add_queue,
      kind: "action",
      action: { QueueTrack: track },
    });

  const go: MenuEntry[] = [];
  if (song && track.id)
    go.push({
      id: "radio",
      label: s.go_song_radio,
      kind: "open",
      view: { page: "radio", id: track.id, name: track.name },
    });
  if (spotify) {
    const artists: MenuEntry[] = track.artist_refs.flatMap((artist) =>
      artist.id
        ? [
            {
              id: artist.id,
              label: artist.name,
              kind: "open" as const,
              view: { page: "artist", id: artist.id, name: artist.name },
            },
          ]
        : [],
    );
    if (artists.length === 1) go.push({ ...artists[0], label: s.go_artist });
    else if (artists.length > 1)
      go.push({
        id: "artists",
        label: s.go_artist,
        kind: "submenu",
        entries: artists,
      });
    if (track.album_id)
      go.push({
        id: "album",
        label: s.go_album,
        kind: "open",
        view: { page: "album", id: track.album_id },
      });
  }

  const links: MenuEntry[] = [];
  if (song) {
    links.push({
      id: "copy-song",
      label: s.copy_song_link,
      kind: "copy",
      text: `https://open.spotify.com/track/${track.id}`,
      done: "copied_song_link",
    });
    if (track.album_id)
      links.push({
        id: "copy-album",
        label: s.copy_album_link,
        kind: "copy",
        text: `https://open.spotify.com/album/${track.album_id}`,
        done: "copied_album_link",
      });
  }
  const share: MenuEntry[] =
    links.length > 0
      ? [{ id: "share", label: s.share, kind: "submenu", entries: links }]
      : [];

  return [edit, go, share].filter((group) => group.length > 0);
}
