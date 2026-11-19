export const DAY_MS = 24 * 60 * 60 * 1000;

/** Soll beim Start nach Updates gesucht werden? Höchstens einmal pro Tag und nur, wenn die Einstellung an ist. */
export function shouldAutoCheck(now: number, lastCheck: number | null, enabled: boolean): boolean {
  if (!enabled) return false;
  if (lastCheck === null || lastCheck > now) return true;
  return now - lastCheck >= DAY_MS;
}

export const LAST_CHECK_KEY = "daedyx.updates.lastCheck";
export const AUTO_KEY = "daedyx.updates.auto";

export function readLastCheck(): number | null {
  try {
    const raw = localStorage.getItem(LAST_CHECK_KEY);
    const n = raw === null ? NaN : Number(raw);
    return Number.isFinite(n) ? n : null;
  } catch {
    return null;
  }
}

export function writeLastCheck(now: number): void {
  try {
    localStorage.setItem(LAST_CHECK_KEY, String(now));
  } catch {
    // gesperrt: dann eben beim nächsten Start wieder
  }
}
