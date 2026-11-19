import { useCallback, useEffect, useRef, useState, type ChangeEvent } from "react";

/**
 * Eingabefeld mit eigenem Entwurf: Tippen ändert nur den Entwurf, gespeichert (`commit`) wird nach einer kurzen
 * Pause oder sofort beim Verlassen des Feldes. So baut sich das Feld beim Tippen nicht um (Datum, Uhrzeit).
 */
export function useDebouncedField(value: string, commit: (v: string) => void, delay = 700) {
  const [draft, setDraft] = useState(value);
  const pending = useRef<string | null>(null);
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const commitRef = useRef(commit);
  commitRef.current = commit;

  useEffect(() => {
    if (pending.current === null) setDraft(value);
  }, [value]);

  const flush = useCallback(() => {
    clearTimeout(timer.current);
    if (pending.current === null) return;
    const v = pending.current;
    pending.current = null;
    commitRef.current(v);
  }, []);

  useEffect(() => flush, [flush]);

  const onChange = (e: ChangeEvent<HTMLInputElement>) => {
    const v = e.target.value;
    setDraft(v);
    pending.current = v;
    clearTimeout(timer.current);
    timer.current = setTimeout(flush, delay);
  };

  return { value: draft, onChange, onBlur: flush };
}
