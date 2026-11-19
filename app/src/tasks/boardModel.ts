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

/** Je Oberaufgabe (erledigt, gesamt) aller Unteraufgaben, auch der tieferen Ebenen. */
export function subtaskProgress(tasks: readonly Task[]): Map<string, [number, number]> {
  const byId = new Map(tasks.map((t) => [t.id, t]));
  const m = new Map<string, [number, number]>();
  for (const k of tasks) {
    const seen = new Set<string>();
    for (let p = k.parentId ? byId.get(k.parentId) : undefined; p && !seen.has(p.id); p = p.parentId ? byId.get(p.parentId) : undefined) {
      seen.add(p.id);
      const [d, n] = m.get(p.id) ?? [0, 0];
      m.set(p.id, [d + (k.doneAt !== null ? 1 : 0), n + 1]);
    }
  }
  return m;
}

/** Ebene jeder Aufgabe in der Ansicht (0 = oberste). Fehlt die Oberaufgabe in der Ansicht, zählt die Aufgabe als oberste. */
export function taskDepths(tasks: readonly Task[]): Map<string, number> {
  const byId = new Map(tasks.map((t) => [t.id, t]));
  const depth = (t: Task, seen: Set<string>): number => {
    const parent = t.parentId ? byId.get(t.parentId) : undefined;
    if (!parent || seen.has(parent.id)) return 0;
    return 1 + depth(parent, seen.add(t.id));
  };
  return new Map(tasks.map((t) => [t.id, depth(t, new Set())]));
}

/** IDs der Abschnitte nach dem Verschieben von `movedId` vor `beforeId` (null = ans Ende). */
export function reorderedSectionIds(sections: readonly Section[], movedId: string, beforeId: string | null): string[] {
  const ids = [...sections].sort((a, b) => a.position - b.position).map((s) => s.id);
  if (movedId === beforeId) return ids;
  const rest = ids.filter((id) => id !== movedId);
  const at = beforeId === null ? -1 : rest.indexOf(beforeId);
  return at < 0 ? [...rest, movedId] : [...rest.slice(0, at), movedId, ...rest.slice(at)];
}
