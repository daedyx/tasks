import { t } from "../i18n";

/** Auswahl beim Hinzufügen: Minuten vor der Fälligkeit und Textschlüssel. */
export const ALARM_PRESETS: readonly [number, string][] = [
  [0, "tasks.reminder.at_due"],
  [5, "tasks.reminder.min_5"],
  [30, "tasks.reminder.min_30"],
  [60, "tasks.reminder.hour_1"],
  [1440, "tasks.reminder.day_1"],
  [2880, "tasks.reminder.day_2"],
  [10080, "tasks.reminder.week_1"],
];

export const MAX_ALARMS = 10;

/** Lesbarer Text zu einer Erinnerung; Werte außerhalb der Auswahl (z. B. aus einer anderen App) als Minuten. */
export function alarmLabel(minutes: number): string {
  const hit = ALARM_PRESETS.find(([m]) => m === minutes);
  return hit ? t(hit[1]) : t("tasks.reminder.min_custom", { n: minutes });
}
