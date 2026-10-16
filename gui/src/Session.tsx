import { EN, type GuiStrings, fmt, sourceLabel, sourceBadge } from "./i18n";
import { Fragment, useEffect, useState, type KeyboardEvent } from "react";
import type { Action } from "./bindings/Action";
import type { NowPlaying } from "./bindings/NowPlaying";
import type { QueuePayload } from "./bindings/QueuePayload";
import type { SessionPlay } from "./bindings/SessionPlay";
import type { TrackInfo } from "./bindings/TrackInfo";
import type { Position } from "./connection";
import { clock, sourceOf } from "./format";
import { KeyHints } from "./KeyHints";
import { clampCursor } from "./libraryModel";
import { type QueueCursor, queueKey, queueRow } from "./queueModel";
import "./Session.css";
import {
  estimates,
  headline,
  markers,
  positionAt,
  replayRun,
  timeOfDay,
  type Marker,
} from "./sessionModel";
import { SourceBadge } from "./SourceBadge";
import { Cover } from "./ui";
import { usePosition } from "./usePosition";

const PAST_COVERS = 5;
const NEXT_COVERS = 5;

/** This listening session as a line: what played, what plays, what comes next. */
export function Session({
  s = EN,
  plays,
  item,
  position,
  queue,
  upNext,
  active,
  send,
}: {
  s?: GuiStrings;
  plays: SessionPlay[];
  item: NowPlaying | null;
  position: Position | null;
  queue: QueuePayload | null;
  upNext: TrackInfo[];
  active: boolean;
  send: (action: Action) => void;
}) {
  const running = item?.is_playing ?? false;
  // A hidden Session keeps no animation frame running.
  const ms = usePosition(position, running && active);
  const [inspect, setInspect] = useState<Marker | null>(null);
  // The Earlier cursor names its play, so a new finished play does not move it to another.
  const [earlierAt, setEarlierAt] = useState<number | null>(null);
  const [nextWant, setNextWant] = useState<QueueCursor>({
    index: 0,
    uri: null,
  });
  // The estimates move by the minute; a clock in state keeps the render pure. The wall clock
  // and the page clock are sampled together, so the position below is taken at that same moment.
  const [now, setNow] = useState(() => ({
    wall: Date.now(),
    page: performance.now(),
  }));
  useEffect(() => {
    const timer = window.setInterval(
      () => setNow({ wall: Date.now(), page: performance.now() }),
      30_000,
    );
    return () => window.clearInterval(timer);
  }, []);
  const sampledMs = positionAt(position, running, now.page);

  const native = queue?.native ?? [];
  const found = markers(plays);
  const recent = plays.slice(-PAST_COVERS);
  const firstShown = plays.length - recent.length;
  const starts = estimates(
    {
      durationMs: item?.is_live ? 0 : (item?.duration_ms ?? 0),
      positionMs: sampledMs,
      running,
    },
    now.wall,
    upNext,
  );
  const newestFirst = [...plays].reverse();
  const earlier = Math.max(
    newestFirst.findIndex((play) => play.started_at_ms === earlierAt),
    newestFirst.length > 0 ? 0 : -1,
  );
  const pickEarlier = (row: number) => {
    const play = newestFirst[clampCursor(row, newestFirst.length)];
    if (play) setEarlierAt(play.started_at_ms);
  };
  const next = queueRow(native, nextWant);

  const onEarlierKey = (event: KeyboardEvent<HTMLOListElement>) => {
    if (event.ctrlKey || event.metaKey || event.altKey) return;
    if (event.key === "Enter") {
      const action = replayRun(plays, plays.length - 1 - earlier);
      if (action) send(action);
    } else if (event.key === "j" || event.key === "ArrowDown")
      pickEarlier(earlier + 1);
    else if (event.key === "k" || event.key === "ArrowUp")
      pickEarlier(earlier - 1);
    else return;
    event.preventDefault();
  };

  return (
    <div className="session" tabIndex={-1}>
      <div className="session-head">
        <h1>{s.this_session}</h1>
        <span>{headline(plays, item?.uri ?? null, s)}</span>
        <kbd>{s.session_keys}</kbd>
      </div>

      <div className="timeline" aria-label={s.timeline}>
        <div className="lane past">
          <span className="label">{s.played}</span>
          <div className="covers">
            {firstShown > 0 && (
              <span className="more">
                +{firstShown} {s.more}
              </span>
            )}
            {recent.map((entry, index) => {
              const at = firstShown + index;
              const marks = found.filter((mark) => mark.before === at);
              return (
                <Fragment key={`${at}-${entry.started_at_ms}`}>
                  {marks.map((marker, stack) => (
                    <button
                      key={`${marker.kind}-${stack}`}
                      type="button"
                      className={`marker ${marker.kind}`}
                      style={{ marginTop: stack * 14 }}
                      aria-label={markerTitle(marker, s)}
                      onClick={() => setInspect(marker)}
                    >
                      <span>{markerLabel(marker, s)}</span>
                    </button>
                  ))}
                  <CoverCard
                    image={entry.image_url}
                    title={entry.title}
                    meta={entry.artists.join(", ")}
                    fade={0.55 + index * 0.1}
                  />
                </Fragment>
              );
            })}
            {found
              .filter((mark) => mark.before === plays.length)
              .map((marker, stack) => (
                <button
                  key={`last-${marker.kind}-${stack}`}
                  type="button"
                  className={`marker ${marker.kind}`}
                  style={{ marginTop: stack * 14 }}
                  aria-label={markerTitle(marker, s)}
                  onClick={() => setInspect(marker)}
                >
                  <span>{markerLabel(marker, s)}</span>
                </button>
              ))}
          </div>
        </div>
        <div className="lane now">
          <span className="label">{s.now_playing}</span>
          {item ? (
            <div className="now-block">
              {item.image_url ? (
                <img src={item.image_url} alt="" />
              ) : (
                <span className="tile" />
              )}
              <b>{item.title}</b>
              <span>
                {item.artists.join(", ")} ·{" "}
                {clock(Math.min(ms, item.duration_ms || ms))} /{" "}
                {item.is_live ? s.live : clock(item.duration_ms)}
              </span>
            </div>
          ) : (
            <p className="empty">{s.nothing_is_playing}</p>
          )}
        </div>
        <div className="lane upcoming">
          <span className="label">{s.coming_up}</span>
          <div className="covers">
            {upNext.slice(0, NEXT_COVERS).map((track, index) => (
              <CoverCard
                key={`${index}-${track.uri ?? track.name}`}
                image={track.image_url}
                title={track.name}
                meta={
                  starts[index] !== null
                    ? `≈ ${timeOfDay(starts[index] as number)}`
                    : track.artists.join(", ")
                }
                fade={1}
              />
            ))}
          </div>
        </div>
      </div>

      {inspect && (
        <div
          className="marker-dialog"
          role="dialog"
          aria-label={s.marker_details}
        >
          <span className="label">{markerHeading(inspect, plays, s)}</span>
          <span>{markerDetail(inspect, plays, s)}</span>
          <button type="button" onClick={() => setInspect(null)}>
            {s.close}
          </button>
        </div>
      )}

      <div className="session-lists">
        <section aria-label={s.earlier}>
          <div className="section-head">
            <h2>{s.earlier}</h2>
            <kbd>{s.replay_hint}</kbd>
          </div>
          {newestFirst.length === 0 ? (
            <p className="empty">{s.nothing_finished}</p>
          ) : (
            <ol
              role="listbox"
              aria-label={s.earlier}
              aria-activedescendant={`earlier-${earlier}`}
              tabIndex={0}
              data-focus
              onKeyDown={onEarlierKey}
            >
              {newestFirst.map((entry, index) => {
                const source = sourceOf(entry.uri);
                return (
                  <li
                    key={`${index}-${entry.started_at_ms}`}
                    id={`earlier-${index}`}
                    role="option"
                    aria-selected={index === earlier}
                    onClick={() => pickEarlier(index)}
                    onDoubleClick={() => {
                      const action = replayRun(plays, plays.length - 1 - index);
                      if (action) send(action);
                    }}
                  >
                    <span className="at">{timeOfDay(entry.started_at_ms)}</span>
                    <Cover src={entry.image_url} size={40} />
                    <span className="name">
                      <b>{entry.title}</b>
                      <span>{entry.artists.join(", ")}</span>
                    </span>
                    {source && <SourceBadge s={s} source={source} />}
                  </li>
                );
              })}
            </ol>
          )}
        </section>
        <section aria-label={s.up_next}>
          <div className="section-head">
            <h2>{s.up_next}</h2>
            <kbd>{s.queue_keys}</kbd>
          </div>
          {upNext.length === 0 ? (
            <p className="empty">{s.nothing_queued}</p>
          ) : (
            <>
              {native.length > 0 && (
                <ol
                  role="listbox"
                  aria-label={s.your_queue}
                  aria-activedescendant={`session-q-${next}`}
                  tabIndex={0}
                  data-focus={newestFirst.length === 0 ? true : undefined}
                  onKeyDown={(event) => {
                    if (event.ctrlKey || event.metaKey || event.altKey) return;
                    const press = queueKey(event.key, next, native);
                    if (!press) return;
                    event.preventDefault();
                    setNextWant(press.cursor);
                    if (press.action) send(press.action);
                  }}
                >
                  {native.map((track, index) => (
                    <NextRow
                      s={s}
                      key={`${index}-${track.uri ?? track.name}`}
                      id={`session-q-${index}`}
                      track={track}
                      start={starts[index] ?? null}
                      selected={index === next}
                      movable
                      onPick={() => setNextWant({ index, uri: track.uri })}
                    />
                  ))}
                </ol>
              )}
              <ol className="mirror">
                {upNext.slice(native.length).map((track, index) => (
                  <NextRow
                    s={s}
                    key={`${index}-${track.uri ?? track.name}`}
                    track={track}
                    start={starts[native.length + index] ?? null}
                    selected={false}
                    movable={false}
                  />
                ))}
              </ol>
            </>
          )}
        </section>
      </div>
      <KeyHints
        hints={[s.hint_move, s.hint_replay_play, s.hint_reorder, s.hint_remove]}
      />
    </div>
  );
}

function CoverCard({
  image,
  title,
  meta,
  fade,
}: {
  image: string | null;
  title: string;
  meta: string;
  fade: number;
}) {
  return (
    <div className="cover-card" style={{ opacity: Math.min(fade, 1) }}>
      {image ? <img src={image} alt="" /> : <span className="tile" />}
      <b>{title}</b>
      <span>{meta}</span>
    </div>
  );
}

function NextRow({
  s = EN,
  id,
  track,
  start,
  selected,
  movable,
  onPick,
}: {
  s?: GuiStrings;
  id?: string;
  track: TrackInfo;
  start: number | null;
  selected: boolean;
  movable: boolean;
  onPick?: () => void;
}) {
  const source = sourceOf(track.uri);
  return (
    <li
      id={id}
      role={movable ? "option" : undefined}
      aria-selected={movable ? selected : undefined}
      onClick={onPick}
    >
      <span className="grip" aria-hidden="true">
        {movable ? "⋮⋮" : ""}
      </span>
      <span className="at">
        {start !== null ? `≈ ${timeOfDay(start)}` : ""}
      </span>
      <Cover src={track.image_url} size={40} />
      <span className="name">
        <b>{track.name}</b>
        <span>{track.artists.join(", ")}</span>
      </span>
      {source && <SourceBadge s={s} source={source} />}
    </li>
  );
}

function markerLabel(marker: Marker, s: GuiStrings = EN): string {
  if (marker.kind === "source")
    return `${marker.from ? sourceBadge(marker.from, s) : "?"} → ${marker.to ? sourceBadge(marker.to, s) : "?"}`;
  return marker.kind === "gap"
    ? fmt(s.gap_minutes, { minutes: marker.minutes ?? "undefined" })
    : fmt(s.paused_minutes, { minutes: marker.minutes ?? "undefined" });
}

function markerTitle(marker: Marker, s: GuiStrings = EN): string {
  if (marker.kind === "pause")
    return fmt(s.paused_about, { minutes: marker.minutes ?? "undefined" });
  const what = marker.kind === "source" ? s.source_change : s.gap;
  return fmt(s.marker_at, { what, time: timeOfDay(marker.at) });
}

/** A pause inside a play has a length but no known clock time, so it names the song. */
function markerHeading(
  marker: Marker,
  plays: SessionPlay[],
  s: GuiStrings = EN,
): string {
  if (marker.kind === "pause")
    return fmt(s.paused_during, {
      title: plays[marker.before - 1]?.title ?? s.a_song,
    });
  const what = marker.kind === "source" ? s.source_change : s.gap;
  return `${what} · ${timeOfDay(marker.at)}`;
}

function markerDetail(
  marker: Marker,
  plays: SessionPlay[],
  s: GuiStrings = EN,
): string {
  const after = plays[marker.before];
  if (marker.kind === "pause")
    return fmt(s.pause_detail, { minutes: marker.minutes ?? "undefined" });
  if (marker.kind === "gap")
    return fmt(s.gap_detail, {
      minutes: marker.minutes ?? "undefined",
      after: after ? fmt(s.then_played, { title: after.title }) : "",
    });
  return fmt(s.source_detail, {
    from: marker.from ? sourceLabel(marker.from, s) : "undefined",
    to: marker.to ? sourceLabel(marker.to, s) : "undefined",
    after: after
      ? fmt(s.played_from, {
          title: after.title,
          source: marker.to ? sourceLabel(marker.to, s) : "undefined",
        })
      : "",
  });
}
