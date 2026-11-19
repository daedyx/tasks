import { useCallback, useEffect, useState } from "react";

function read(key: string, def: number, min: number, max: number): number {
  try {
    const raw = localStorage.getItem(key);
    if (raw === null || raw.trim() === "") return def;
    const n = Number(raw);
    if (!Number.isFinite(n)) return def;
    return Math.min(max, Math.max(min, Math.round(n)));
  } catch {
    return def;
  }
}

export function usePaneWidth(storageKey: string, defaultPx: number, min: number, max: number) {
  const [width, setW] = useState(() => read(storageKey, defaultPx, min, max));

  useEffect(() => {
    try {
      localStorage.setItem(storageKey, String(width));
    } catch {
      // Speicher gesperrt: Breite gilt nur bis zum Schließen
    }
  }, [storageKey, width]);

  const setWidth = useCallback((n: number) => setW(Math.min(max, Math.max(min, Math.round(n)))), [min, max]);
  const reset = useCallback(() => setW(defaultPx), [defaultPx]);

  return { width, setWidth, reset, min, max };
}
