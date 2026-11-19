import { describe, expect, it } from "vitest";
import { DAY_MS, shouldAutoCheck } from "./autoCheck";

describe("shouldAutoCheck", () => {
  const now = 1_000_000_000_000;

  it("sucht, wenn nie gesucht wurde", () => {
    expect(shouldAutoCheck(now, null, true)).toBe(true);
  });

  it("sucht höchstens einmal pro Tag", () => {
    expect(shouldAutoCheck(now, now - DAY_MS + 1000, true)).toBe(false);
    expect(shouldAutoCheck(now, now - DAY_MS, true)).toBe(true);
  });

  it("sucht nie, wenn die Einstellung aus ist", () => {
    expect(shouldAutoCheck(now, null, false)).toBe(false);
  });

  it("ein Zeitpunkt in der Zukunft (Uhr verstellt) zählt als nie gesucht", () => {
    expect(shouldAutoCheck(now, now + 5 * DAY_MS, true)).toBe(true);
  });
});
