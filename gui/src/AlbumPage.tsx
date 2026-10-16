import { useStringsReader } from "./useStringsReader";
import { EN, type GuiStrings, fmt } from "./i18n";
import { useState, type KeyboardEvent } from "react";
import type { Action } from "./bindings/Action";
import type { AlbumInfo } from "./bindings/AlbumInfo";
import { Glyph } from "./Icon";
import { clampCursor } from "./libraryModel";
import type { OpenView } from "./nav";
import "./Pages.css";
import {
  albumKind,
  albumPlay,
  copyLink,
  lengthLine,
  releaseLine,
  trackKey,
  yearOf,
} from "./pagesModel";
import { TrackTable } from "./TrackTable";
import { PageHeader, PlayButton } from "./ui";

/** An album page; `album` is the last fetched album, which can still be another one. */
export function AlbumPage({
  s = EN,
  id,
  album,
  saved,
  playingUri,
  open,
  send,
}: {
  s?: GuiStrings;
  id: string;
  album: AlbumInfo | null;
  /** The album is in the user's library. */
  saved: boolean;
  playingUri: string | null;
  open: OpenView;
  send: (action: Action) => void;
}) {
  const readStrings = useStringsReader(s);
  const [want, setWant] = useState(0);
  const [shownId, setShownId] = useState(id);
  if (shownId !== id) {
    setShownId(id);
    setWant(0);
  }

  if (!album || album.id !== id)
    return (
      <section className="album-page">
        <PageHeader
          kind={s.album}
          title={s.loading}
          image={null}
          tone="album"
        />
      </section>
    );

  const tracks = album.tracks;
  const cursor = clampCursor(want, tracks.length);
  const play = (index: number) => {
    const action = albumPlay(album, index);
    if (action) send(action);
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.ctrlKey || event.metaKey || event.altKey) return;
    const key = trackKey(event.key, cursor, tracks.length);
    if (!key) return;
    event.preventDefault();
    if (key.kind === "play") play(cursor);
    else if (key.kind === "move") setWant(key.next);
    else if (tracks[cursor] && !event.repeat)
      send({ QueueTrack: tracks[cursor] });
  };
  const year = yearOf(album.release_date);
  const released = releaseLine(album.release_date, s);
  const albumId = album.id;
  const saveLabel = saved ? s.remove_from_library : s.save_to_library;

  return (
    <section className="album-page">
      <PageHeader
        kind={albumKind(album.album_type, s)}
        title={album.name}
        image={album.image_url}
        tone="album"
        meta={
          <>
            {album.artists.map((artist, index) => (
              <span key={`${index}-${artist.name}`}>
                {index > 0 && ", "}
                {artist.id ? (
                  <button
                    type="button"
                    className="meta-link"
                    onClick={() =>
                      artist.id &&
                      open({ page: "artist", id: artist.id, name: artist.name })
                    }
                  >
                    {artist.name}
                  </button>
                ) : (
                  artist.name
                )}
              </span>
            ))}
            {year && <span className="meta-dot">{year}</span>}
            <span className="meta-dot">{lengthLine(tracks, s)}</span>
          </>
        }
      />
      <div className="page-actions">
        <PlayButton
          label={fmt(s.play_named, { name: album.name })}
          large
          disabled={tracks.length === 0}
          onClick={() => play(0)}
        />
        <button
          type="button"
          className={saved ? "icon-action on" : "icon-action"}
          aria-label={saveLabel}
          title={saveLabel}
          aria-pressed={saved}
          disabled={!albumId}
          onClick={() =>
            albumId &&
            send(saved ? { UnsaveAlbum: albumId } : { SaveAlbum: albumId })
          }
        >
          <Glyph name="heart" size={32} filled={saved} />
        </button>
        <button
          type="button"
          className="icon-action"
          aria-label={s.copy_album_link}
          title={s.copy_album_link}
          disabled={!albumId}
          onClick={() =>
            albumId &&
            void copyLink(
              `https://open.spotify.com/album/${albumId}`,
              "copied_album_link",
              send,
              navigator.clipboard,
              readStrings,
            )
          }
        >
          <Glyph name="link" size={28} />
        </button>
      </div>
      <div className="page-tracks">
        <TrackTable
          s={s}
          label={album.name}
          idPrefix="album"
          tracks={tracks}
          cursor={cursor}
          playingUri={playingUri}
          showAlbum={false}
          showCover={false}
          onKeyDown={onKeyDown}
          onPick={setWant}
          onPlay={play}
          open={open}
        />
      </div>
      {released && <p className="page-footnote">{released}</p>}
    </section>
  );
}
