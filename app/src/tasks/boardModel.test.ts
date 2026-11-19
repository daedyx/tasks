import { describe, expect, it } from "vitest";
import { boardColumns, reorderedSectionIds, subtaskProgress } from "./boardModel";
import type { Section, Task } from "./types";

let n = 0;
const task = (title: string, extra: Partial<Task> = {}): Task => ({
  id: `t${++n}`, listId: "l", parentId: null, sectionId: null, title, notes: "", due: null, dueHasTime: false, priority: 0, doneAt: null,
  position: n, repeat: null, remindMinutes: null, assigneeId: null, createdBy: null, ...extra,
});
const sec = (id: string, position: number): Section => ({ id, listId: "l", name: `Abschnitt ${id}`, position });
const titles = (ts: Task[]) => ts.map((t) => t.title);

describe("Board-Spalten", () => {
  it("zeigt ohne Abschnitte nur „Ohne Abschnitt“, auch wenn sie leer ist", () => {
    const cols = boardColumns([], []);
    expect(cols).toHaveLength(1);
    expect(cols[0]).toMatchObject({ id: null, cards: [] });
  });

  it("ordnet Karten nach Abschnitt und Handreihenfolge, Erledigte unten", () => {
    const a = task("A", { sectionId: "s1", position: 2 });
    const b = task("B", { sectionId: "s1", position: 1 });
    const c = task("C", { sectionId: "s1", position: 0, doneAt: 5 });
    const d = task("D", { sectionId: "s2" });
    const cols = boardColumns([a, b, c, d], [sec("s2", 2), sec("s1", 1)]);
    expect(cols.map((x) => x.id)).toEqual(["s1", "s2"]);
    expect(titles(cols[0].cards)).toEqual(["B", "A", "C"]);
    expect(titles(cols[1].cards)).toEqual(["D"]);
  });

  it("blendet „Ohne Abschnitt“ erst ein, wenn dort Karten liegen", () => {
    const loose = task("Lose");
    const cols = boardColumns([loose], [sec("s1", 1)]);
    expect(cols.map((x) => x.id)).toEqual([null, "s1"]);
    expect(titles(cols[0].cards)).toEqual(["Lose"]);
  });

  it("legt Karten eines unbekannten (gelöschten) Abschnitts nach „Ohne Abschnitt“", () => {
    const orphan = task("Waise", { sectionId: "weg" });
    const cols = boardColumns([orphan], [sec("s1", 1)]);
    expect(cols[0]).toMatchObject({ id: null });
    expect(titles(cols[0].cards)).toEqual(["Waise"]);
  });

  it("lässt Unteraufgaben aus den Spalten und zählt sie an der Oberaufgabe", () => {
    const p = task("Eltern", { sectionId: "s1" });
    const k1 = task("K1", { parentId: p.id, doneAt: 3 });
    const k2 = task("K2", { parentId: p.id });
    const cols = boardColumns([p, k1, k2], [sec("s1", 1)]);
    expect(titles(cols[0].cards)).toEqual(["Eltern"]);
    expect(subtaskProgress([p, k1, k2]).get(p.id)).toEqual([1, 2]);
  });
});

describe("Reihenfolge der Abschnitte", () => {
  const s = [sec("a", 1), sec("b", 2), sec("c", 3)];
  it("setzt einen Abschnitt vor einen anderen", () => {
    expect(reorderedSectionIds(s, "c", "a")).toEqual(["c", "a", "b"]);
    expect(reorderedSectionIds(s, "a", "c")).toEqual(["b", "a", "c"]);
  });
  it("ans Ende bei fehlendem Ziel und unverändert bei sich selbst", () => {
    expect(reorderedSectionIds(s, "a", null)).toEqual(["b", "c", "a"]);
    expect(reorderedSectionIds(s, "b", "b")).toEqual(["a", "b", "c"]);
  });
});
