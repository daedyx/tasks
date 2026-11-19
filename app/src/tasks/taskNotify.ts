import { t } from "../i18n";
import type { TaskReminder } from "./types";

/** Text der Benachrichtigung zu einer Erinnerung. */
export function reminderBody(r: TaskReminder): string {
  const m = r.minutesBefore;
  if (m <= 0) return t("tasks.reminder.body_now");
  if (m < 60) return t("tasks.reminder.body_in_minutes", { n: m });
  if (m < 1440) return m < 120 ? t("tasks.reminder.body_in_hour_one") : t("tasks.reminder.body_in_hours", { n: Math.round(m / 60) });
  return m < 2880 ? t("tasks.reminder.body_in_day_one") : t("tasks.reminder.body_in_days", { n: Math.round(m / 1440) });
}

const isTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

/** Zeigt die Windows-Benachrichtigung (nur in der App; fragt einmal nach der Erlaubnis). Fehler bleiben still. */
export async function showReminder(r: TaskReminder): Promise<boolean> {
  if (!isTauri()) return false;
  try {
    const { isPermissionGranted, requestPermission, sendNotification } = await import("@tauri-apps/plugin-notification");
    const granted = (await isPermissionGranted()) || (await requestPermission()) === "granted";
    if (!granted) return false;
    sendNotification({ title: r.title || t("tasks.reminder.untitled"), body: reminderBody(r) });
    return true;
  } catch {
    return false;
  }
}
