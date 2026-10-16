import { EN, type GuiStrings, fmt } from "./i18n";
import type { StatsMovement } from "./bindings/StatsMovement";
import type { StatsRow } from "./bindings/StatsRow";

/** The period ring of the app, in the order `[` and `]` step through it. */
export const periods = (
  s: GuiStrings = EN,
): { key: string; label: string; phrase: string }[] => [
  { key: "7d", label: s.seven_days, phrase: s.in_last_seven_days },
  { key: "30d", label: s.thirty_days, phrase: s.in_last_thirty_days },
  { key: "month", label: s.this_month, phrase: s.this_month_phrase },
  { key: "year", label: s.this_year, phrase: s.this_year_phrase },
  { key: "all", label: s.all_time, phrase: s.of_all_time },
];
export const PERIODS = periods();

/** How many steps, and which way, from one period to another on the ring. */
export function cyclesTo(
  from: string,
  to: string,
): { forward: boolean; count: number } {
  const size = PERIODS.length;
  const start = PERIODS.findIndex((period) => period.key === from);
  const end = PERIODS.findIndex((period) => period.key === to);
  if (start < 0 || end < 0) return { forward: true, count: 0 };
  const ahead = (end - start + size) % size;
  return ahead <= size - ahead
    ? { forward: true, count: ahead }
    : { forward: false, count: size - ahead };
}

export type Tone = "up" | "down" | "same";

/** An artist's place against its all-time rank. `rank` is 1-based within the period. */
export function moveText(
  row: StatsRow,
  rank: number,
  s: GuiStrings = EN,
): { text: string; tone: Tone } {
  if (row.new) return { text: s.new, tone: "up" };
  const before = row.all_time_rank;
  if (before === null || before === rank) return { text: "=", tone: "same" };
  return before > rank
    ? {
        text: fmt(s.rank_up, { count: before - rank, rank: before }),
        tone: "up",
      }
    : {
        text: fmt(s.rank_down, { count: rank - before, rank: before }),
        tone: "down",
      };
}

/** A movement as a sentence in three parts; the middle one is set in amber. */
export function sentence(
  movement: StatsMovement,
  periodKey: string,
  s: GuiStrings = EN,
): [string, string, string] {
  const phrase =
    periods(s).find((period) => period.key === periodKey)?.phrase ?? "";
  const { name, from, to } = movement;
  const template =
    movement.kind === "climb"
      ? s.rank_climbed
      : movement.kind === "fall"
        ? to === null
          ? s.rank_no_plays
          : s.rank_fell
        : s.rank_new;
  const highlight =
    movement.kind === "new"
      ? s.rank_new_marker
      : to === null
        ? s.no_plays
        : `#${to}`;
  // Split the template before interpolation so literal braces in names stay literal.
  const [before, after] = template.split("{highlight}");
  const values = { name, from: from ?? "null", period: phrase };
  return [fmt(before, values), highlight, fmt(after, values)];
}

/** Thousands apart with a no-break space, as the design sets big numbers: `1 043`. */
export function groupThousands(value: number): string {
  return String(value).replace(/\B(?=(\d{3})+(?!\d))/g, "\u00a0");
}

/** The terminal plays only Spotify tracks from Stats. */
export function playable(uri: string | null): uri is string {
  return uri?.startsWith("spotify:track:") ?? false;
}
