import { t } from "../i18n";
import { dayText, dueDay, dueTime } from "./dueDay";
import type { Task } from "./types";

const short = new Intl.DateTimeFormat("de-DE", { weekday: "short", day: "numeric", month: "short" });
const withYear = new Intl.DateTimeFormat("de-DE", { day: "numeric", month: "short", year: "numeric" });

/** Kurzer Text zur Fälligkeit („Heute 18:30“, „Mo., 23. Nov.“) und ob sie überfällig ist; null ohne Fälligkeit. */
export function dueLabel(task: Pick<Task, "due" | "dueHasTime" | "doneAt">, now: Date): { text: string; overdue: boolean } | null {
  const day = dueDay(task);
  if (!day) return null;
  const today = dayText(now);
  const tomorrow = dayText(new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1));
  const yesterday = dayText(new Date(now.getFullYear(), now.getMonth(), now.getDate() - 1));
  const [y, m, d] = day.split("-").map(Number);
  const date = new Date(y, m - 1, d);
  let text: string;
  if (day === today) text = t("tasks.due.today");
  else if (day === tomorrow) text = t("tasks.due.tomorrow");
  else if (day === yesterday) text = t("tasks.due.yesterday");
  else text = (y === now.getFullYear() ? short : withYear).format(date);
  const time = dueTime(task);
  return { text: time ? `${text} ${time}` : text, overdue: task.doneAt === null && day < today };
}
