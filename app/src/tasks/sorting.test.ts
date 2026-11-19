import { describe, expect, it } from "vitest";
import { positionBefore, positionBetween, sortTasks } from "./sorting";
import type { Task } from "./types";

let n = 0;
const task = (title: string, extra: Partial<Task> = {}): Task => ({
  id: `t${++n}`, listId: "l", parentId: null, sectionId: null, title, notes: "", due: null, dueHasTime: false, priority: 0, doneAt: null,
  position: n, repeat: null, remindMinutes: null, assigneeId: null, createdBy: null, ...extra,
});
const titles = (ts: Task[]) => ts.map((t) => t.title);

describe("Aufgaben sortieren", () => {
  it("Handreihenfolge nach Position, Unteraufgaben direkt nach der Oberaufgabe", () => {
    const a = task("A");
    const b = task("B");
    const a1 = task("A1", { parentId: a.id });
    expect(titles(sortTasks([a1, b, a], "hand"))).toEqual(["A", "A1", "B"]);
  });

  it("nach Fälligkeit, ohne Datum zuletzt", () => {
    const ts = [task("ohne"), task("spät", { due: "2026-12-24" }), task("früh", { due: "2026-11-01" })];
    expect(titles(sortTasks(ts, "due"))).toEqual(["früh", "spät", "ohne"]);
  });

  it("nach Priorität, hohe zuerst", () => {
    const ts = [task("a", { priority: 1 }), task("b", { priority: 3 }), task("c", { priority: 0 })];
    expect(titles(sortTasks(ts, "priority"))).toEqual(["b", "a", "c"]);
  });

  it("nach Titel ohne Beachtung von Groß- und Kleinschreibung und mit Umlauten", () => {
    const ts = [task("Zebra"), task("ärger"), task("Apfel")];
    expect(titles(sortTasks(ts, "title"))).toEqual(["Apfel", "ärger", "Zebra"]);
  });

  it("Unteraufgabe ohne Oberaufgabe in der Ansicht zählt als oberste", () => {
    const orphan = task("Waise", { parentId: "fehlt" });
    expect(titles(sortTasks([orphan], "hand"))).toEqual(["Waise"]);
  });

  it("Position zwischen Nachbarn und am Rand", () => {
    expect(positionBetween(null, null)).toBe(1);
    expect(positionBetween(1, 2)).toBe(1.5);
    expect(positionBetween(null, 3)).toBe(2);
    expect(positionBetween(5, null)).toBe(6);
  });

  it("Position vor einer Aufgabe, am Anfang und am Ende", () => {
    const a = task("A", { position: 1 });
    const b = task("B", { position: 2 });
    const c = task("C", { position: 4 });
    expect(positionBefore([a, b, c], b.id)).toBe(1.5);
    expect(positionBefore([a, b, c], a.id)).toBe(0);
    expect(positionBefore([a, b, c], null)).toBe(5);
    expect(positionBefore([], null)).toBe(1);
    expect(positionBefore([a], "unbekannt")).toBe(2);
  });
});
