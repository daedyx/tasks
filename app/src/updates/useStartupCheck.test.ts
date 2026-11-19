import { renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LAST_CHECK_KEY } from "./autoCheck";
import type { UpdaterApi } from "./updater";
import { useStartupCheck } from "./useStartupCheck";

function fake(check: UpdaterApi["check"]): UpdaterApi {
  return { currentVersion: async () => "0.4.0", check, install: async () => undefined, restart: async () => undefined };
}

describe("useStartupCheck", () => {
  beforeEach(() => localStorage.clear());

  it("liefert die gefundene Version und merkt sich den Zeitpunkt", async () => {
    const api = fake(async () => ({ version: "0.4.1", notes: "" }));
    const { result } = renderHook(() => useStartupCheck(api));
    await waitFor(() => expect(result.current?.version).toBe("0.4.1"));
    expect(localStorage.getItem(LAST_CHECK_KEY)).not.toBeNull();
  });

  it("sucht am selben Tag nicht noch einmal", async () => {
    localStorage.setItem(LAST_CHECK_KEY, String(Date.now()));
    const check = vi.fn(async () => null);
    renderHook(() => useStartupCheck(fake(check)));
    await new Promise((r) => setTimeout(r, 20));
    expect(check).not.toHaveBeenCalled();
  });

  it("sucht nicht, wenn die Einstellung aus ist", async () => {
    localStorage.setItem("daedyx.updates.auto", "0");
    const check = vi.fn(async () => null);
    renderHook(() => useStartupCheck(fake(check)));
    await new Promise((r) => setTimeout(r, 20));
    expect(check).not.toHaveBeenCalled();
  });

  it("ein Fehler bleibt stumm", async () => {
    const check = vi.fn(async () => Promise.reject(new Error("kein Netz")));
    const { result } = renderHook(() => useStartupCheck(fake(check)));
    await waitFor(() => expect(check).toHaveBeenCalled());
    expect(result.current).toBeNull();
  });
});
