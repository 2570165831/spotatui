import { EN, type GuiStrings, fmt } from "./i18n";
import type { Action } from "./bindings/Action";
import type { SettingInput } from "./bindings/SettingInput";
import type { SettingRow } from "./bindings/SettingRow";
import type { UpdateAsk } from "./bindings/UpdateAsk";
import type { UpdateStatus } from "./bindings/UpdateStatus";
import { albumSorts, artistSorts } from "./libraryModel";
import { sortOptions } from "./sortModel";

/** The page's sections, in display order. */
export const groups = (s: GuiStrings = EN) =>
  [
    { group: "interface", heading: s.interface },
    { group: "playback", heading: s.playback },
    { group: "library", heading: s.library },
    { group: "sources", heading: s.sources },
    { group: "streaming", heading: s.streaming },
    { group: "integrations", heading: s.integrations },
    { group: "updates", heading: s.updates },
  ] as const;
export const GROUPS = groups();

export interface Section {
  group: string;
  heading: string;
  rows: SettingRow[];
}

/** The sections with rows that hold every word of `query`; Updates shows with an updater and no query. */
export function sections(
  rows: SettingRow[],
  updater: boolean,
  query = "",
  s: GuiStrings = EN,
): Section[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
  return groups(s)
    .map(({ group, heading }) => ({
      group,
      heading,
      rows: rows.filter(
        (row) => row.group === group && matches(row, heading, terms),
      ),
    }))
    .filter((section) =>
      section.group === "updates"
        ? updater && (terms.length === 0 || section.rows.length > 0)
        : section.rows.length > 0,
    );
}

function matches(row: SettingRow, heading: string, terms: string[]): boolean {
  const haystack = `${heading} ${row.name} ${row.description}`.toLowerCase();
  return terms.every((term) => haystack.includes(term));
}

const optionLabels = (
  s: GuiStrings,
): Record<string, { value: string; label: string }[]> => ({
  "behavior.default_sort_playlist_tracks": sortOptions(s),
  "behavior.default_sort_saved_albums": albumSorts(s),
  "behavior.default_sort_saved_artists": artistSorts(s),
});

/** The label a choice row shows for `option`; the option itself when it has none. */
export function optionLabel(
  id: string,
  option: string,
  s: GuiStrings = EN,
): string {
  return (
    optionLabels(s)[id]?.find((known) => known.value === option)?.label ??
    option
  );
}

export function setSetting(id: string, value: SettingInput): Action {
  return { SetSetting: { id, value } };
}

/** The value a field commits; null when it is unchanged, not a whole number, or an empty password. */
export function commitValue(
  row: SettingRow,
  draft: string,
): SettingInput | null {
  if (row.value.kind === "number") {
    const text = draft.trim();
    const number = Number(text);
    if (text === "" || !Number.isSafeInteger(number)) return null;
    return number === row.value.value ? null : { Number: number };
  }
  if (row.value.kind === "text")
    return draft === row.value.value ? null : { Text: draft };
  // An empty Text clears a saved password, so a blank field sends nothing.
  if (row.value.kind === "secret") return draft === "" ? null : { Text: draft };
  return null;
}

export interface UpdateLine {
  text: string;
  button: { label: string; ask: UpdateAsk; disabled: boolean } | null;
  failed: boolean;
}

/** The Updates section's status line and its one action. */
export function updateLine(
  status: UpdateStatus,
  version: string | null,
  s: GuiStrings = EN,
): UpdateLine {
  const line = (
    text: string,
    button: UpdateLine["button"] = null,
    failed = false,
  ): UpdateLine => ({ text, button, failed });
  switch (status.state) {
    case "idle":
    case "up_to_date":
      return line(
        version ? fmt(s.latest_version_number, { version }) : s.latest_version,
        { label: s.check_updates, ask: "check", disabled: false },
      );
    case "checking":
      return line(s.looking_updates, {
        label: s.check_updates,
        ask: "check",
        disabled: true,
      });
    case "available":
      return line(fmt(s.version_available, { version: status.version }), {
        label: s.download,
        ask: "download",
        disabled: false,
      });
    case "downloading":
      return line(
        status.percent === null
          ? fmt(s.downloading_version, { version: status.version })
          : fmt(s.downloading_percent, {
              version: status.version,
              percent: Math.round(status.percent),
            }),
      );
    case "ready":
      return line(fmt(s.version_ready, { version: status.version }), {
        label: s.restart_update,
        ask: "install",
        disabled: false,
      });
    case "manual":
      return line(fmt(s.version_manual, { version: status.version }));
    case "failed":
      return line(
        status.message,
        { label: s.try_again, ask: "check", disabled: false },
        true,
      );
  }
}

/** The version the top bar offers to install: only a downloaded one. */
export function readyVersion(status: UpdateStatus | null): string | null {
  return status?.state === "ready" ? status.version : null;
}
