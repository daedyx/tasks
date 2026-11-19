import { useCallback, useState } from "react";

/** Ein Ja/Nein-Zustand, der in `localStorage` überlebt (Speicher gesperrt: gilt bis zum Schließen). */
export function useStoredFlag(key: string, initial: boolean): [boolean, (value: boolean) => void] {
  const [value, setValue] = useState(() => {
    try {
      const raw = localStorage.getItem(key);
      return raw === null ? initial : raw === "1";
    } catch {
      return initial;
    }
  });
  const set = useCallback(
    (next: boolean) => {
      setValue(next);
      try {
        localStorage.setItem(key, next ? "1" : "0");
      } catch {
        // gesperrt: nur für diese Sitzung
      }
    },
    [key],
  );
  return [value, set];
}
