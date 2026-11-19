import { useEffect } from "react";
import { onTaskReminder } from "./api";
import { showReminder } from "./taskNotify";

/** Zeigt Erinnerungen zu Aufgaben als Benachrichtigung, solange die App läuft. */
export function useTaskReminders(): void {
  useEffect(() => {
    let on = true;
    let off: (() => void) | undefined;
    void onTaskReminder((r) => void showReminder(r)).then((u) => (on ? (off = u) : u()));
    return () => {
      on = false;
      off?.();
    };
  }, []);
}
