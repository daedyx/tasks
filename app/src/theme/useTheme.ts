import { useCallback, useLayoutEffect, useState } from "react";

export type ThemeMode = "system" | "light" | "dark";

const STORAGE_KEY = "daedyx.theme";
const NEXT: Record<ThemeMode, ThemeMode> = { system: "light", light: "dark", dark: "system" };

export function applyTheme(mode: ThemeMode, root: HTMLElement = document.documentElement): void {
  if (mode === "system") root.removeAttribute("data-theme");
  else root.setAttribute("data-theme", mode);
}

function readStored(): ThemeMode {
  try {
    const value = localStorage.getItem(STORAGE_KEY);
    return value === "light" || value === "dark" ? value : "system";
  } catch {
    return "system";
  }
}

export function useTheme(): { mode: ThemeMode; cycle: () => void } {
  const [mode, setMode] = useState<ThemeMode>(readStored);

  useLayoutEffect(() => {
    applyTheme(mode);
    try {
      localStorage.setItem(STORAGE_KEY, mode);
    } catch {
      // Speicher gesperrt: Wahl gilt nur bis zum Schließen
    }
  }, [mode]);

  const cycle = useCallback(() => setMode((m) => NEXT[m]), []);
  return { mode, cycle };
}
