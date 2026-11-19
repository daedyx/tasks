import { beforeEach, describe, expect, it, vi } from "vitest";
import { KEEP_RUNNING_KEY, readKeepRunning, writeKeepRunning } from "./backgroundPrefs";

describe("backgroundPrefs", () => {
  beforeEach(() => localStorage.clear());

  it("ist ohne gespeicherten Wert an (Erinnerungen sollen nicht am geschlossenen Fenster scheitern)", () => {
    expect(readKeepRunning()).toBe(true);
  });

  it("speichert und liest den Wert", () => {
    writeKeepRunning(false);
    expect(localStorage.getItem(KEEP_RUNNING_KEY)).toBe("0");
    expect(readKeepRunning()).toBe(false);
    writeKeepRunning(true);
    expect(readKeepRunning()).toBe(true);
  });

  it("fällt bei gesperrtem Speicher auf an zurück", () => {
    const spy = vi.spyOn(Storage.prototype, "getItem").mockImplementation(() => {
      throw new Error("gesperrt");
    });
    expect(readKeepRunning()).toBe(true);
    spy.mockRestore();
  });
});
