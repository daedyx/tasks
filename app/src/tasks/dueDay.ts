import type { Task } from "./types";

const pad = (n: number): string => String(n).padStart(2, "0");
export const dayText = (d: Date): string => `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;

/** Kalendertag der Fälligkeit (Ortszeit) als `YYYY-MM-DD`, oder null ohne Fälligkeit. */
export function dueDay(task: Pick<Task, "due" | "dueHasTime">): string | null {
  if (!task.due) return null;
  if (!task.dueHasTime) return task.due;
  const secs = Number(task.due);
  return Number.isFinite(secs) ? dayText(new Date(secs * 1000)) : null;
}

/** Uhrzeit `HH:MM` bei Fälligkeit mit Uhrzeit, sonst null. */
export function dueTime(task: Pick<Task, "due" | "dueHasTime">): string | null {
  if (!task.due || !task.dueHasTime) return null;
  const d = new Date(Number(task.due) * 1000);
  return `${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

/** Fälligkeitsfeld aus Tag und optionaler Uhrzeit (Ortszeit); leerer Tag = keine Fälligkeit. */
export function makeDue(day: string, time: string): { due: string | null; due_has_time: boolean } {
  if (!day) return { due: null, due_has_time: false };
  if (!time) return { due: day, due_has_time: false };
  const [y, m, d] = day.split("-").map(Number);
  const [hh, mm] = time.split(":").map(Number);
  return { due: String(Math.floor(new Date(y, m - 1, d, hh, mm).getTime() / 1000)), due_has_time: true };
}
