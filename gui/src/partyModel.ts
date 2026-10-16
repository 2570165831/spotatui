import { EN, type GuiStrings, fmt } from "./i18n";
import type { Action } from "./bindings/Action";
import type { PartyPayload } from "./bindings/PartyPayload";
import type { PartyRoom } from "./bindings/PartyRoom";

/** The relay's room codes are six characters of A-Z and 0-9. */
export const CODE_LENGTH = 6;
const NAME_LIMIT = 32;

export function normalizeCode(input: string): string {
  return input
    .toUpperCase()
    .replace(/[^A-Z0-9]/g, "")
    .slice(0, CODE_LENGTH);
}

export function normalizeName(input: string): string {
  return input.slice(0, NAME_LIMIT);
}

export function joinRequest(code: string, name: string): Action | null {
  const trimmed = name.trim();
  return code.length === CODE_LENGTH && trimmed
    ? { JoinParty: { code, name: trimmed } }
    : null;
}

/** "You host · 3 listening" counts the host; a guest names the room it follows. */
export function headline(party: PartyPayload, s: GuiStrings = EN): string {
  const room = party.room;
  if (!room) return party.phase === "connecting" ? s.connecting : "";
  if (room.host)
    return fmt(s.host_listening, { count: room.guests.length + 1 });
  return (
    fmt(s.follow_host, { name: room.host_name || s.the_host }) +
    (room.code ? fmt(s.room_code_suffix, { code: room.code }) : "")
  );
}

export interface Listener {
  name: string;
  role: string;
  initial: string;
  hue: number;
}

/** You first, then the others; a guest learns only who joined after it. */
export function listeners(room: PartyRoom, s: GuiStrings = EN): Listener[] {
  const others = room.host
    ? room.guests.map((name) => ({ name, role: s.guest_role }))
    : [
        { name: room.host_name || s.host, role: s.host_role },
        ...room.guests.map((name) => ({ name, role: s.guest_role })),
      ];
  return [
    { name: s.you, role: room.host ? s.host_role : s.guest_role },
    ...others,
  ].map((listener) => ({
    ...listener,
    initial: listener.name.charAt(0).toUpperCase() || "?",
    hue: hue(listener.name),
  }));
}

function hue(name: string): number {
  let hash = 0;
  for (const char of name) hash = (hash * 31 + char.charCodeAt(0)) % 360;
  return hash;
}

/**
 * Guests follow the host only while a Spotify track plays outside the native
 * queue: a queue slot or a decoded source owns the sink and the relay stops.
 */
export function relaying(
  room: PartyRoom,
  queueNow: boolean,
  playingUri: string | null,
): boolean {
  return (
    room.host && !queueNow && /^spotify:(track|episode):/.test(playingUri ?? "")
  );
}
