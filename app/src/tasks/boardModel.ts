// Reine Rechenlogik des Boards (ohne Oberfläche): Spalten, Zähler, Reihenfolge der Abschnitte.
import type { Section, Task } from "./types";

export type Column = { id: string | null; section: Section | null; cards: Task[] };

/** Offene Karten zuerst (Handreihenfolge), Erledigte darunter. */
const cardOrder = (a: Task, b: Task): number => Number(a.doneAt !== null) - Number(b.doneAt !== null) || a.position - b.position;

/**
 * Spalten eines Boards. „Ohne Abschnitt“ (id null) steht vorn, solange dort Karten liegen oder es keine Abschnitte gibt.
 * Karten eines unbekannten (z. B. gleichzeitig gelöschten) Abschnitts landen in „Ohne Abschnitt“; Unteraufgaben sind keine Karten.
 */
export function boardColumns(tasks: readonly Task[], sections: readonly Section[]): Column[] {
  const sorted = [...sections].sort((a, b) => a.position - b.position);
  const known = new Set(sorted.map((s) => s.id));
  const top = tasks.filter((t) => !t.parentId);
  const inColumn = (id: string | null) => top.filter((t) => (t.sectionId && known.has(t.sectionId) ? t.sectionId : null) === id).sort(cardOrder);
  const loose = inColumn(null);
  const columns: Column[] = sorted.map((s) => ({ id: s.id, section: s, cards: inColumn(s.id) }));
  return loose.length > 0 || sorted.length === 0 ? [{ id: null, section: null, cards: loose }, ...columns] : columns;
}

/** Je Oberaufgabe (erledigt, gesamt) der Unteraufgaben. */
export function subtaskProgress(tasks: readonly Task[]): Map<string, [number, number]> {
  const m = new Map<string, [number, number]>();
  for (const k of tasks) {
    if (!k.parentId) continue;
    const [d, n] = m.get(k.parentId) ?? [0, 0];
    m.set(k.parentId, [d + (k.doneAt !== null ? 1 : 0), n + 1]);
  }
  return m;
}

/** IDs der Abschnitte nach dem Verschieben von `movedId` vor `beforeId` (null = ans Ende). */
export function reorderedSectionIds(sections: readonly Section[], movedId: string, beforeId: string | null): string[] {
  const ids = [...sections].sort((a, b) => a.position - b.position).map((s) => s.id);
  if (movedId === beforeId) return ids;
  const rest = ids.filter((id) => id !== movedId);
  const at = beforeId === null ? -1 : rest.indexOf(beforeId);
  return at < 0 ? [...rest, movedId] : [...rest.slice(0, at), movedId, ...rest.slice(at)];
}
