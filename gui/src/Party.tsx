import { useStringsReader } from "./useStringsReader";
import { copyLink } from "./pagesModel";
import { EN, type GuiStrings } from "./i18n";
import { useEffect, useState, type KeyboardEvent } from "react";
import type { Action } from "./bindings/Action";
import type { NowPlaying } from "./bindings/NowPlaying";
import type { PartyPayload } from "./bindings/PartyPayload";
import type { PartyRoom } from "./bindings/PartyRoom";
import type { QueuePayload } from "./bindings/QueuePayload";
import type { Position } from "./connection";
import { clock, sourceOf } from "./format";
import { KeyHints } from "./KeyHints";
import "./Party.css";
import {
  headline,
  joinRequest,
  listeners,
  normalizeCode,
  normalizeName,
  relaying,
} from "./partyModel";
import { type QueueCursor, queueKey, queueRow } from "./queueModel";
import { Cover } from "./ui";
import { usePosition } from "./usePosition";

/** The listening party: host or join, then the room with the listeners and the host's queue. */
export function Party({
  s = EN,
  party,
  connected,
  item,
  position,
  queue,
  active,
  send,
}: {
  s?: GuiStrings;
  party: PartyPayload | null;
  connected: boolean;
  item: NowPlaying | null;
  position: Position | null;
  queue: QueuePayload | null;
  active: boolean;
  send: (action: Action) => void;
}) {
  const room = party?.room ?? null;
  // Starting, joining or leaving swaps the view under the focused button; the new view takes the keyboard.
  const view = room ? "room" : "start";
  useEffect(() => {
    if (!active) return;
    document
      .querySelector<HTMLElement>('[data-area="party"] [data-focus]')
      ?.focus();
  }, [view, active]);
  if (!party || !room)
    return <Start s={s} party={party} connected={connected} send={send} />;
  return (
    <Room
      s={s}
      party={party}
      room={room}
      item={item}
      position={position}
      queue={queue}
      active={active}
      send={send}
    />
  );
}

function Start({
  s = EN,
  party,
  connected,
  send,
}: {
  s?: GuiStrings;
  party: PartyPayload | null;
  connected: boolean;
  send: (action: Action) => void;
}) {
  const [code, setCode] = useState("");
  const [name, setName] = useState("");
  const available = party?.available ?? false;
  const connecting = party?.phase === "connecting";
  const join = joinRequest(code, name);
  // A click waits for the next party push, so a double click cannot open two rooms. Without a
  // socket the click would be dropped, so both forms wait; the resync push releases the wait.
  const [clickedFor, setClickedFor] = useState<PartyPayload | null>(null);
  const pending = clickedFor !== null && clickedFor === party;
  const disabled = !connected || !available || connecting || pending;
  return (
    <div className="party" data-focus tabIndex={-1}>
      <div className="party-head">
        <h1>{s.listening_party}</h1>
        <span>{connecting ? s.connecting : s.party_intro}</span>
      </div>
      {!available && <p className="notice-line">{s.party_needs_spotify}</p>}
      <div className="party-start">
        <section aria-label={s.host_party}>
          <span className="label">{s.host}</span>
          <h2>{s.start_party}</h2>
          <p>{s.party_host_intro}</p>
          <button
            type="button"
            className="primary"
            disabled={disabled}
            onClick={() => {
              setClickedFor(party);
              send("StartParty");
            }}
          >
            {s.start_party}
          </button>
        </section>
        <section aria-label={s.join_party}>
          <span className="label">{s.join}</span>
          <h2>{s.join_party}</h2>
          <form
            onSubmit={(event) => {
              event.preventDefault();
              if (join && !disabled) {
                setClickedFor(party);
                send(join);
              }
            }}
          >
            <label>
              <span>{s.code}</span>
              <input
                type="text"
                value={code}
                placeholder="ABC123"
                autoComplete="off"
                disabled={disabled}
                onChange={(event) => setCode(normalizeCode(event.target.value))}
              />
            </label>
            <label>
              <span>{s.your_name}</span>
              <input
                type="text"
                value={name}
                placeholder={s.name_placeholder}
                disabled={disabled}
                onChange={(event) => setName(normalizeName(event.target.value))}
              />
            </label>
            <button type="submit" disabled={disabled || !join}>
              {s.join}
            </button>
          </form>
        </section>
      </div>
    </div>
  );
}

function Room({
  s = EN,
  party,
  room,
  item,
  position,
  queue,
  active,
  send,
}: {
  s?: GuiStrings;
  party: PartyPayload;
  room: PartyRoom;
  item: NowPlaying | null;
  position: Position | null;
  queue: QueuePayload | null;
  active: boolean;
  send: (action: Action) => void;
}) {
  const readStrings = useStringsReader(s);
  const [confirming, setConfirming] = useState(false);
  const [want, setWant] = useState<QueueCursor>({ index: 0, uri: null });
  const playingUri = item?.uri ?? null;
  const live = relaying(room, queue?.now != null, playingUri);
  // A guest follows only while its own sink plays the Spotify track the host sent.
  const following =
    room.host ||
    (queue?.now == null && /^spotify:(track|episode):/.test(playingUri ?? ""));
  const native = queue?.native ?? [];
  const mirror =
    sourceOf(playingUri) === "Spotify"
      ? (queue?.spotify.items ?? []).flatMap((entry) =>
          entry.track ? [entry.track] : [],
        )
      : [];
  const cursor = queueRow(native, want);
  const elapsed = usePosition(position, (item?.is_playing ?? false) && active);

  const copy = () => {
    if (!room.code) return;
    void copyLink(
      room.code,
      "copy_party_done",
      send,
      navigator.clipboard,
      readStrings,
      "copy_party_failed",
    );
  };
  const leave = () => {
    if (room.host && !confirming) setConfirming(true);
    else send("LeaveParty");
  };

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (
      event.defaultPrevented ||
      event.ctrlKey ||
      event.metaKey ||
      event.altKey
    )
      return;
    if (
      event.target instanceof HTMLInputElement &&
      event.target.type !== "checkbox"
    )
      return;
    if (event.key === "y") copy();
    else if (event.key === "L") leave();
    else if (event.key === "c" && room.host) send("TogglePartyControlMode");
    else if (event.key === "Escape" && confirming) setConfirming(false);
    else return;
    event.preventDefault();
  };

  return (
    <div className="party" tabIndex={-1} data-focus onKeyDown={onKeyDown}>
      <div className="party-head">
        <h1>{s.listening_party}</h1>
        <span>{headline(party, s)}</span>
        {room.code ? (
          <button type="button" className="code" onClick={copy}>
            {room.code} <kbd>{s.copy_hint}</kbd>
          </button>
        ) : (
          <span className="code">{s.getting_code}</span>
        )}
        {confirming ? (
          <span className="confirm" role="group" aria-label={s.end_party_label}>
            {s.end_party_question}
            <button type="button" className="danger" onClick={leave}>
              {s.end_party}
            </button>
            <button
              type="button"
              autoFocus
              onClick={() => setConfirming(false)}
            >
              {s.keep_it}
            </button>
          </span>
        ) : (
          <button type="button" onClick={leave}>
            {room.host ? s.end_party : s.leave_party}
          </button>
        )}
      </div>
      <div className="party-room">
        <section aria-label={s.everyone_hears}>
          <span className="label">
            {following ? s.everyone_hears : s.you_hear}
          </span>
          <div className="hears">
            <Cover src={item?.image_url} size={120} />
            <div>
              <span className="song">{item?.title ?? s.nothing_playing}</span>
              {item && (
                <span className="meta">
                  {[item.artists.join(", "), item.album]
                    .filter(Boolean)
                    .join(" · ")}{" "}
                  · {clock(Math.min(elapsed, item.duration_ms || elapsed))} /{" "}
                  {clock(item.duration_ms)}
                </span>
              )}
            </div>
          </div>
          {room.host && !live && (
            <p className="hint">{s.spotify_follow_only}</p>
          )}
          {!following && <p className="hint">{s.stopped_following}</p>}
          <span className="label">{s.each_own_copy}</span>
          <ul className="listeners">
            {listeners(room, s).map((listener, index) => (
              <li key={`${index}-${listener.name}`}>
                <span
                  className="avatar-dot"
                  style={{ background: `hsl(${listener.hue} 45% 70%)` }}
                >
                  {listener.initial}
                </span>
                <b>{listener.name}</b>
                <span>{listener.role}</span>
              </li>
            ))}
          </ul>
          {room.host && (
            <label className="control">
              <input
                type="checkbox"
                checked={room.shared_control}
                onChange={() => send("TogglePartyControlMode")}
              />
              {s.guest_controls} <kbd>c</kbd>
            </label>
          )}
        </section>
        <section aria-label={s.up_next}>
          <div className="section-head">
            <h2>{s.up_next}</h2>
            {room.host && <kbd>{s.queue_keys}</kbd>}
          </div>
          {!room.host ? (
            <p className="empty">{s.host_picks_songs}</p>
          ) : native.length === 0 && mirror.length === 0 ? (
            <p className="empty">{s.nothing_queued}</p>
          ) : (
            <>
              <div className="qrow head" aria-hidden="true">
                <span>#</span>
                <span />
                <span>{s.song}</span>
                <span>{s.guests}</span>
              </div>
              {native.length > 0 && (
                <ol
                  role="listbox"
                  aria-label={s.your_queue}
                  aria-activedescendant={`party-q-${cursor}`}
                  tabIndex={0}
                  onKeyDown={(event) => {
                    if (event.ctrlKey || event.metaKey || event.altKey) return;
                    const press = queueKey(event.key, cursor, native);
                    if (!press) return;
                    event.preventDefault();
                    setWant(press.cursor);
                    if (press.action) send(press.action);
                  }}
                >
                  {native.map((track, index) => (
                    <li
                      key={`${index}-${track.uri ?? track.name}`}
                      id={`party-q-${index}`}
                      role="option"
                      aria-selected={index === cursor}
                      className="qrow"
                      onClick={() => setWant({ index, uri: track.uri })}
                    >
                      <span className="n">{index + 1}</span>
                      <Cover src={track.image_url} size={40} />
                      <span className="name">
                        <b>{track.name}</b>
                        <span>{track.artists.join(", ")}</span>
                      </span>
                      <span className="only">{s.only_you}</span>
                    </li>
                  ))}
                </ol>
              )}
              <ol className="mirror">
                {mirror.map((track, index) => (
                  <li
                    key={`${index}-${track.uri ?? track.name}`}
                    className="qrow"
                  >
                    <span className="n">{native.length + index + 1}</span>
                    <Cover src={track.image_url} size={40} />
                    <span className="name">
                      <b>{track.name}</b>
                      <span>{track.artists.join(", ")}</span>
                    </span>
                    <span>{s.guests_follow}</span>
                  </li>
                ))}
              </ol>
            </>
          )}
        </section>
      </div>
      <KeyHints
        hints={
          room.host
            ? [s.hint_copy_code, s.hint_guest_control, s.hint_end_party]
            : [s.hint_copy_code, s.hint_leave]
        }
      />
    </div>
  );
}
