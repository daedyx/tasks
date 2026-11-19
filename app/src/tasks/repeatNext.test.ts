import { describe, expect, it } from "vitest";
import { nextDue } from "./repeatNext";
import type { RepeatRule } from "./types";

const monthly = (extra: Partial<RepeatRule> = {}): RepeatRule => ({ kind: "monthly", interval: 1, weekdays: [], fromDone: false, ...extra });

describe("nextDue", () => {
  it("monatlich ohne festen Tag nimmt den Tag der Fälligkeit, kurze Monate den letzten Tag", () => {
    expect(nextDue(monthly(), "2026-01-15")).toBe("2026-02-15");
    expect(nextDue(monthly(), "2026-01-31")).toBe("2026-02-28");
  });

  it("monatlich mit festem Tag springt auf diesen Tag, auch wenn die Fälligkeit anders lag", () => {
    expect(nextDue(monthly({ monthDay: 15 }), "2026-01-03")).toBe("2026-02-15");
    expect(nextDue(monthly({ monthDay: 15 }), "2026-01-20")).toBe("2026-02-15");
  });

  it("der Anker bleibt nach einem kurzen Monat erhalten (31. → 28. Feb. → 31. März)", () => {
    const feb = nextDue(monthly({ monthDay: 31 }), "2026-01-31");
    expect(feb).toBe("2026-02-28");
    expect(nextDue(monthly({ monthDay: 31 }), feb)).toBe("2026-03-31");
    expect(nextDue(monthly({ monthDay: 31 }), "2028-01-31")).toBe("2028-02-29");
  });

  it("beachtet den Abstand in Monaten und das Jahr", () => {
    expect(nextDue(monthly({ interval: 3, monthDay: 1 }), "2026-11-20")).toBe("2027-02-01");
    expect(nextDue({ kind: "yearly", interval: 1, weekdays: [], fromDone: false, monthDay: 29 }, "2027-02-28")).toBe("2028-02-29");
  });

  it("täglich und wöchentlich bleiben wie bisher", () => {
    expect(nextDue({ kind: "daily", interval: 2, weekdays: [], fromDone: false }, "2026-12-31")).toBe("2027-01-02");
    expect(nextDue({ kind: "weekly", interval: 1, weekdays: [], fromDone: false }, "2026-01-01")).toBe("2026-01-08");
  });
});
