import {
  createContext,
  useCallback,
  useContext,
  useLayoutEffect,
  useRef,
} from "react";
import type { GuiStrings } from "./i18n";

export type StringsReader = () => GuiStrings;

/** Read the connection's current table even after the initiating view closes. */
export const StringsReaderContext = createContext<StringsReader | null>(null);

export function useStringsReader(fallback: GuiStrings): StringsReader {
  const read = useContext(StringsReaderContext);
  const latest = useRef(fallback);
  useLayoutEffect(() => {
    latest.current = fallback;
  }, [fallback]);
  return useCallback(() => (read ? read() : latest.current), [read]);
}
