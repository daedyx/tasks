import { describe, expect, it } from "vitest";
import { dueDay, dueTime, makeDue } from "./dueDay";

describe("Fälligkeit", () => {
  it("ohne Fälligkeit gibt es keinen Tag", () => {
    expect(dueDay({ due: null, dueHasTime: false })).toBeNull();
    expect(dueTime({ due: null, dueHasTime: false })).toBeNull();
  });

  it("ein Datum bleibt, ohne Uhrzeit", () => {
    expect(dueDay({ due: "2026-12-24", dueHasTime: false })).toBe("2026-12-24");
    expect(dueTime({ due: "2026-12-24", dueHasTime: false })).toBeNull();
  });

  it("Tag und Uhrzeit lassen sich in Ortszeit hin und zurück rechnen", () => {
    const { due, due_has_time } = makeDue("2026-12-24", "18:30");
    expect(due_has_time).toBe(true);
    expect(dueDay({ due, dueHasTime: true })).toBe("2026-12-24");
    expect(dueTime({ due, dueHasTime: true })).toBe("18:30");
  });

  it("leerer Tag löscht, Tag ohne Uhrzeit ist ganztägig", () => {
    expect(makeDue("", "09:00")).toEqual({ due: null, due_has_time: false });
    expect(makeDue("2026-12-24", "")).toEqual({ due: "2026-12-24", due_has_time: false });
  });

  it("unlesbare Sekunden ergeben keinen Tag", () => {
    expect(dueDay({ due: "abc", dueHasTime: true })).toBeNull();
  });
});
