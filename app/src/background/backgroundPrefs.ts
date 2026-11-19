export const KEEP_RUNNING_KEY = "daedyx.background.keepRunning";

/** Soll die App beim Schließen des Fensters im Infobereich weiterlaufen (Erinnerungen)? Standard: ja. */
export function readKeepRunning(): boolean {
  try {
    return localStorage.getItem(KEEP_RUNNING_KEY) !== "0";
  } catch {
    return true;
  }
}

export function writeKeepRunning(on: boolean): void {
  try {
    localStorage.setItem(KEEP_RUNNING_KEY, on ? "1" : "0");
  } catch {
    // gesperrt: gilt dann nur bis zum Neustart
  }
}
