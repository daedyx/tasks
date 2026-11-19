// Nächste Fälligkeit im Fake-Kern der Oberfläche; spiegelt `repeat.rs` des Kerns (Monatstag-Anker, Monatsende).
import { dayText } from "./dueDay";
import type { RepeatRule } from "./types";

const lastDay = (y: number, m0: number): number => new Date(y, m0 + 1, 0).getDate();

/** Tag `day` im Monat `m0` (0-basiert) des Jahres `y`; zu große Tage ergeben den letzten Tag des Monats. */
function inMonth(y: number, m0: number, day: number): Date {
  const total = y * 12 + m0;
  const yy = Math.floor(total / 12);
  const mm = ((total % 12) + 12) % 12;
  return new Date(yy, mm, Math.min(day, lastDay(yy, mm)));
}

export function nextDue(rule: RepeatRule, day: string): string {
  const [y, m, d] = day.split("-").map(Number);
  const n = Math.max(1, rule.interval);
  const anchor = rule.monthDay ?? d;
  if (rule.kind === "daily") return dayText(new Date(y, m - 1, d + n));
  if (rule.kind === "weekly") return dayText(new Date(y, m - 1, d + 7 * n));
  if (rule.kind === "monthly") return dayText(inMonth(y, m - 1 + n, anchor));
  return dayText(inMonth(y + n, m - 1, anchor));
}
