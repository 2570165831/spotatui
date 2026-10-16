import { EN, type GuiStrings, fmt, sourceLabel } from "./i18n";
import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import type { Action } from "./bindings/Action";
import type { PlaylistSyncPayload } from "./bindings/PlaylistSyncPayload";
import {
  cellFor,
  lastSync,
  reasonText,
  selectedIndex,
  SYNC_SOURCES,
} from "./healthModel";
import { Glyph } from "./Icon";
import { KeyHints } from "./KeyHints";
import "./LibraryHealth.css";
import { SourceIcon } from "./SourceBadge";

const cellText = (s: GuiStrings) => ({
  master: s.sync_master,
  synced: s.synced,
  never: s.never,
  none: "·",
});

/** Library health: the playlists mirrored across sources. The copy tabs need a source resolver that does not exist yet. */
export function LibraryHealth({
  s = EN,
  sync,
  send,
  onBack,
}: {
  s?: GuiStrings;
  sync: PlaylistSyncPayload | null;
  send: (action: Action) => void;
  onBack: () => void;
}) {
  const links = sync?.links ?? [];
  const running = sync?.running ?? false;
  const [id, setId] = useState<string | null>(null);
  // The confirm names its link, so a run that reorders the links cannot redirect it.
  const [confirmId, setConfirmId] = useState<string | null>(null);
  const root = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const list = root.current?.querySelector<HTMLElement>("[data-focus]");
    (list ?? root.current)?.focus();
  }, []);
  const index = selectedIndex(links, id);
  const link = links[index] ?? null;
  const confirming = link !== null && confirmId === link.id;
  const remove = () => {
    if (link) send({ RemovePlaylistSyncLink: link.id });
    setConfirmId(null);
  };

  const pick = (next: number) => {
    const target = links[Math.min(Math.max(next, 0), links.length - 1)];
    if (target) setId(target.id);
    setConfirmId(null);
  };

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.ctrlKey || event.metaKey || event.altKey) return;
    const key = event.key;
    // A focused button acts on its own Enter and space.
    if (
      event.target instanceof HTMLButtonElement &&
      (key === "Enter" || key === " ")
    )
      return;
    if (confirming && (key === "Enter" || key === "y")) remove();
    else if (key === "Escape" && confirming) setConfirmId(null);
    else if (key === "Escape" || key === "Backspace") onBack();
    else if (key === "j" || key === "ArrowDown") pick(index + 1);
    else if (key === "k" || key === "ArrowUp") pick(index - 1);
    else if (key === "s" && !running) send("RunPlaylistSync");
    else if (key === "D" && link) setConfirmId(link.id);
    else return;
    event.preventDefault();
  };

  return (
    <div ref={root} className="health" tabIndex={-1} onKeyDown={onKeyDown}>
      <section className="health-main">
        <button type="button" className="crumb" onClick={onBack}>
          <Glyph name="back" /> {s.back}
        </button>
        <div className="health-title">
          <h1>{s.library_health}</h1>
          <span>{s.mirrored_playlists}</span>
        </div>
        <div role="tablist" aria-label={s.sections} className="health-tabs">
          <span role="tab" aria-selected="true">
            {s.playlist_sync}
          </span>
        </div>
        {links.length === 0 ? (
          <div className="empty">
            {running ? (
              <p>{s.loading_links}</p>
            ) : (
              <>
                <p>{s.no_links}</p>
                <p>{s.mirror_instructions}</p>
                {sync?.last_failed && sync.last_summary && (
                  <p className="failed">{sync.last_summary}</p>
                )}
              </>
            )}
          </div>
        ) : (
          <>
            <div className="link-row head" aria-hidden="true">
              <span>#</span>
              <span>{s.playlist}</span>
              {SYNC_SOURCES.map((source) => (
                <span key={source}>{source}</span>
              ))}
              <span className="when">{s.last_sync}</span>
            </div>
            <div
              role="listbox"
              aria-label={s.linked_playlists}
              tabIndex={0}
              data-focus
              aria-activedescendant={link ? `link-${link.id}` : undefined}
            >
              {links.map((entry, row) => (
                <div
                  key={entry.id}
                  id={`link-${entry.id}`}
                  role="option"
                  aria-selected={row === index}
                  className="link-row"
                  onClick={() => pick(row)}
                >
                  <span className="n">{row + 1}</span>
                  <span className="name">
                    <SourceIcon source={entry.source} /> <b>{entry.name}</b>
                  </span>
                  {SYNC_SOURCES.map((source) => {
                    const cell = cellFor(entry, source);
                    return (
                      <span key={source} className={`cell ${cell.role}`}>
                        {cell.role === "unmatched"
                          ? fmt(s.unmatched_count, { count: cell.count })
                          : cellText(s)[cell.role]}
                      </span>
                    );
                  })}
                  <span className="when">
                    {entry.mirrors.length === 0
                      ? s.no_mirrors
                      : lastSync(entry, s)}
                  </span>
                </div>
              ))}
            </div>
          </>
        )}
        <KeyHints
          hints={[s.hint_move, s.hint_sync, s.hint_remove_link, s.hint_back]}
        />
      </section>
      <aside aria-label={s.selected_playlist}>
        {link ? (
          <>
            <span className="label">{s.linked_playlist}</span>
            <h2>{link.name}</h2>
            <span className="sub">
              {fmt(
                link.mirrors.length === 1
                  ? s.playlist_one_mirror
                  : s.playlist_mirrors,
                {
                  source: sourceLabel(link.source, s),
                  count: link.mirrors.length,
                },
              )}
            </span>
            {link.mirrors.map((mirror) => (
              <div
                key={mirror.source}
                className={
                  mirror.unmatched.length > 0 ? "mirror-box warn" : "mirror-box"
                }
              >
                <div>
                  <span className="label">
                    <SourceIcon source={mirror.source} />
                    {mirror.source.toUpperCase()}
                  </span>
                  <span className="tag">
                    {mirror.last_run === null
                      ? s.never_ran
                      : mirror.unmatched.length > 0
                        ? fmt(s.unmatched_count, {
                            count: mirror.unmatched.length,
                          })
                        : s.synced}
                  </span>
                </div>
                <span className="detail">
                  {fmt(s.matched_count, { count: mirror.matched })}
                </span>
                {mirror.unmatched.slice(0, 6).map((entry) => (
                  <span key={entry.master_key} className="miss">
                    {entry.title} – {entry.artist}:{" "}
                    {reasonText(entry.reason, s)}
                  </span>
                ))}
              </div>
            ))}
            {link.last_line && (
              <p className={link.last_failed ? "line failed" : "line"}>
                {link.last_line}
              </p>
            )}
            <div className="health-actions">
              <button
                type="button"
                className="primary"
                disabled={running}
                onClick={() => send("RunPlaylistSync")}
              >
                {running ? s.syncing : s.sync_all}
              </button>
              {confirming ? (
                <>
                  <button type="button" className="danger" onClick={remove}>
                    {s.remove_the_link}
                  </button>
                  <button
                    type="button"
                    autoFocus
                    onClick={() => setConfirmId(null)}
                  >
                    {s.keep_it}
                  </button>
                </>
              ) : (
                <button type="button" onClick={() => setConfirmId(link.id)}>
                  {s.remove_link}
                </button>
              )}
            </div>
            {confirming && <p className="sub">{s.mirror_stays}</p>}
          </>
        ) : (
          <span className="sub">{sync?.last_summary ?? s.nothing_synced}</span>
        )}
      </aside>
    </div>
  );
}
