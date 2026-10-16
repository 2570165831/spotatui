import { EN, type GuiStrings, fmt } from "./i18n";
import { useState, type KeyboardEvent } from "react";
import type { Action } from "./bindings/Action";
import type { TrackInfo } from "./bindings/TrackInfo";
import { clampCursor, playRequest, songCount } from "./libraryModel";
import "./ListPage.css";
import type { OpenView } from "./nav";
import "./Pages.css";
import { trackKey } from "./pagesModel";
import { TrackTable } from "./TrackTable";
import { PageHeader, PlayButton } from "./ui";

const NO_TRACKS: TrackInfo[] = [];

export interface RecentlyPlayedProps {
  s?: GuiStrings;
  /** Most recent first; null before the first fetch. */
  recent: TrackInfo[] | null;
  /** A Spotify session exists; without one the list cannot load. */
  available: boolean;
  playingUri: string | null;
  active: boolean;
  open: OpenView;
  send: (action: Action) => void;
}

/** Spotify's recently played tracks. */
export function RecentlyPlayed({
  s = EN,
  recent,
  available,
  playingUri,
  open,
  send,
}: RecentlyPlayedProps) {
  const [want, setWant] = useState(0);
  const tracks = recent ?? NO_TRACKS;
  const cursor = clampCursor(want, tracks.length);
  const play = (index: number) => {
    const action = playRequest(tracks, index);
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
  const title = s.recently_played;
  const first = tracks.findIndex((track) => track.uri);

  return (
    <section className="recent-page" aria-label={title}>
      <PageHeader
        kind={s.history}
        title={title}
        image={null}
        meta={recent && <span>{songCount(recent.length, s)}</span>}
      />
      <div className="page-actions">
        <PlayButton
          label={fmt(s.play_named, { name: title })}
          large
          disabled={first < 0}
          onClick={() => play(first)}
        />
      </div>
      {tracks.length === 0 ? (
        <p className="list-empty">
          {recent
            ? s.nothing_played
            : available
              ? s.loading_recent
              : s.recent_needs_spotify}
        </p>
      ) : (
        <TrackTable
          s={s}
          label={title}
          idPrefix="recent"
          tracks={tracks}
          cursor={cursor}
          playingUri={playingUri}
          onKeyDown={onKeyDown}
          onPick={setWant}
          onPlay={play}
          open={open}
        />
      )}
    </section>
  );
}
