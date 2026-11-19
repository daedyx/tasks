import { describe, expect, it } from "vitest";
import { dueLabel } from "./dueLabel";
import { makeDue } from "./dueDay";

const NOW = new Date(2026, 10, 18, 10, 0);
const open = (due: string | null, dueHasTime = false) => ({ due, dueHasTime, doneAt: null });

describe("Fälligkeit als Text", () => {
  it("ohne Fälligkeit nichts", () => {
    expect(dueLabel(open(null), NOW)).toBeNull();
  });

  it("heute, morgen, gestern", () => {
    expect(dueLabel(open("2026-11-18"), NOW)).toEqual({ text: "Heute", overdue: false });
    expect(dueLabel(open("2026-11-19"), NOW)?.text).toBe("Morgen");
    expect(dueLabel(open("2026-11-17"), NOW)).toEqual({ text: "Gestern", overdue: true });
  });

  it("andere Tage mit Wochentag, in fremden Jahren mit Jahr", () => {
    expect(dueLabel(open("2026-11-23"), NOW)?.text).toMatch(/Mo/);
    expect(dueLabel(open("2027-01-05"), NOW)?.text).toMatch(/2027/);
  });

  it("mit Uhrzeit hängt sie dran", () => {
    const { due } = makeDue("2026-11-18", "18:30");
    expect(dueLabel(open(due, true), NOW)?.text).toBe("Heute 18:30");
  });

  it("erledigte Aufgaben sind nie überfällig", () => {
    expect(dueLabel({ due: "2026-11-01", dueHasTime: false, doneAt: 5 }, NOW)?.overdue).toBe(false);
  });
});
