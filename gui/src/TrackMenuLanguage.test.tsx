import { isValidElement, type ReactNode } from "react";
import { expect, it, vi } from "vitest";
import { MenuList } from "./TrackMenu";
import { trackMenu } from "./trackMenuModel";
import { EN } from "./i18n";
import type { TrackInfo } from "./bindings/TrackInfo";

// Exercise the component's event handlers and render-time state without a browser.
const hooks = vi.hoisted(() => ({ values: [] as unknown[], cursor: 0 }));
vi.mock("react", async (original) => ({
  ...(await original<typeof import("react")>()),
  useRef: () => ({ current: null }),
  useEffect: () => {},
  useLayoutEffect: () => {},
  useState: (initial: unknown) => {
    const index = hooks.cursor++;
    if (!(index in hooks.values)) hooks.values[index] = initial;
    return [
      hooks.values[index],
      (next: unknown) => {
        hooks.values[index] = next;
      },
    ];
  },
}));

function elements(
  node: ReactNode,
): { key?: string | null; type: unknown; props: Record<string, unknown> }[] {
  if (Array.isArray(node)) return node.flatMap(elements);
  if (!isValidElement<Record<string, unknown>>(node)) return [];
  return [node, ...elements(node.props.children as ReactNode)];
}

it("an open share submenu re-derives its title entries and completion text after a language switch", () => {
  hooks.values = [];
  const track = {
    uri: "spotify:track:id",
    id: "id",
    name: "Literal {title}",
    album_id: "album",
    artist_refs: [],
  } as unknown as TrackInfo;
  const translated = {
    ...EN,
    share: "分享",
    copy_song_link: "复制歌曲链接",
    copied_song_link: "歌曲链接已复制",
  };
  const run = vi.fn();
  const render = (s: typeof EN) => {
    hooks.cursor = 0;
    return MenuList({
      label: track.name,
      groups: trackMenu(track, false, [], {}, s),
      at: { x: 0, y: 0 },
      run,
      onClose: () => {},
      root: true,
    });
  };
  const original = elements(render(EN));
  const share = original.find(
    (element) =>
      element.type === "button" &&
      elements(element.props.children as ReactNode).some(
        (child) => child.props.children === EN.share,
      ),
  )!;
  (share.props.onClick as (event: unknown) => void)({
    currentTarget: { getBoundingClientRect: () => ({ right: 30, top: 10 }) },
  });
  const current = elements(render(translated));
  const expanded = current.filter(
    (element) =>
      element.type === "button" && element.props["aria-expanded"] === true,
  );
  expect(expanded).toHaveLength(1);
  expect(
    elements(expanded[0].props.children as ReactNode).some(
      (child) => child.props.children === translated.share,
    ),
  ).toBe(true);
  const sub = current.find((element) => element.type === MenuList)!;
  expect(sub.props.label).toBe(translated.share);
  const entries = sub.props.groups as ReturnType<typeof trackMenu>;
  expect(entries[0][0]).toMatchObject({
    label: translated.copy_song_link,
    text: "https://open.spotify.com/track/id",
    done: "copied_song_link",
  });
  (sub.props.run as typeof run)(entries[0][0]);
  expect(run).toHaveBeenCalledWith(
    expect.objectContaining({ done: "copied_song_link" }),
  );
});

it("all menu button keys survive a language switch so the focused DOM node is retained", () => {
  hooks.values = [];
  const track = {
    uri: "spotify:track:id",
    id: "id",
    name: "Literal title",
    album_id: "album",
    artist_refs: [{ id: "artist", name: "Literal artist" }],
  } as unknown as TrackInfo;
  const alternate = Object.fromEntries(
    Object.entries(EN).map(([key, text]) => [key, `alternate ${text}`]),
  ) as typeof EN;
  const buttons = (s: typeof EN) => {
    hooks.cursor = 0;
    return elements(
      MenuList({
        label: track.name,
        groups: trackMenu(track, false, [], {}, s),
        at: { x: 0, y: 0 },
        run: () => {},
        onClose: () => {},
        root: true,
      }),
    ).filter((node) => node.type === "button");
  };
  const before = buttons(EN),
    after = buttons(alternate);
  expect(before.map((node) => node.key)).toEqual([
    "save",
    "queue",
    "radio",
    "artist",
    "album",
    "share",
  ]);
  expect(after.map((node) => node.key)).toEqual(before.map((node) => node.key));
  expect(after.map((node) => node.props.children)).not.toEqual(
    before.map((node) => node.props.children),
  );
});
