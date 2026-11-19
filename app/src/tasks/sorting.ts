import { dueDay } from "./dueDay";
import type { Task } from "./types";

export type SortMode = "hand" | "due" | "priority" | "title";
export const SORT_MODES: readonly SortMode[] = ["hand", "due", "priority", "title"];

const byTitle = (a: Task, b: Task) => a.title.localeCompare(b.title, "de", { sensitivity: "base" });

function compare(mode: SortMode): (a: Task, b: Task) => number {
  switch (mode) {
    case "hand":
      return (a, b) => a.position - b.position;
    case "due":
      // ohne Fälligkeit ans Ende
      return (a, b) => (dueDay(a) ?? "9999").localeCompare(dueDay(b) ?? "9999") || a.position - b.position;
    case "priority":
      return (a, b) => b.priority - a.priority || a.position - b.position;
    case "title":
      return byTitle;
  }
}

/** Sortiert die obersten Aufgaben nach `mode`; Unteraufgaben bleiben in Handreihenfolge direkt nach ihrer Oberaufgabe. */
export function sortTasks(tasks: readonly Task[], mode: SortMode): Task[] {
  const kids = new Map<string, Task[]>();
  for (const t of tasks) if (t.parentId) kids.set(t.parentId, [...(kids.get(t.parentId) ?? []), t]);
  const ids = new Set(tasks.map((t) => t.id));
  // Unteraufgaben, deren Oberaufgabe nicht in der Ansicht ist (z. B. „Heute“), zählen als oberste
  const top = tasks.filter((t) => !t.parentId || !ids.has(t.parentId)).sort(compare(mode));
  return top.flatMap((t) => [t, ...(kids.get(t.id) ?? []).sort((a, b) => a.position - b.position)]);
}

/** Position zwischen zwei Nachbarn (Bruchzahl); `before`/`after` fehlen am Rand. */
export function positionBetween(before: number | null, after: number | null): number {
  if (before === null && after === null) return 1;
  if (before === null) return (after as number) - 1;
  if (after === null) return before + 1;
  return (before + after) / 2;
}

/** Position, damit eine Aufgabe direkt vor `beforeId` steht (null = ans Ende). `siblings`: Geschwister ohne die verschobene Aufgabe. */
export function positionBefore(siblings: readonly Task[], beforeId: string | null): number {
  const sorted = [...siblings].sort((a, b) => a.position - b.position);
  const at = beforeId === null ? sorted.length : sorted.findIndex((t) => t.id === beforeId);
  if (at < 0) return positionBetween(sorted[sorted.length - 1]?.position ?? null, null);
  return positionBetween(sorted[at - 1]?.position ?? null, sorted[at]?.position ?? null);
}
