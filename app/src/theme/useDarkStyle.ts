import { useCallback, useLayoutEffect, useState } from "react";
import { DEFAULT_DARK_STYLE, applyDarkStyle, normalizeStyle, parseDarkStyle, type DarkStyle } from "./darkStyle";

const STORAGE_KEY = "daedyx.darkStyle";

function readStored(): DarkStyle {
  try {
    return parseDarkStyle(localStorage.getItem(STORAGE_KEY));
  } catch {
    return DEFAULT_DARK_STYLE;
  }
}

export function useDarkStyle() {
  const [style, setStyle] = useState<DarkStyle>(readStored);

  useLayoutEffect(() => {
    applyDarkStyle(style);
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(style));
    } catch {
      // Speicher gesperrt: Wahl gilt nur bis zum Schließen
    }
  }, [style]);

  const update = useCallback((patch: Partial<DarkStyle>) => setStyle((s) => normalizeStyle({ ...s, ...patch })), []);
  const reset = useCallback(() => setStyle(DEFAULT_DARK_STYLE), []);

  return { style, update, reset };
}
