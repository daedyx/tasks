import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { UpdaterApi } from "./updater";
import { useUpdater } from "./useUpdater";

function fake(over: Partial<UpdaterApi> = {}): UpdaterApi {
  return {
    currentVersion: async () => "0.4.0",
    check: async () => null,
    install: async () => undefined,
    restart: vi.fn(async () => undefined),
    ...over,
  };
}

const found = { version: "0.4.1", notes: "Besser" };

describe("useUpdater", () => {
  it("liest die aktuelle Version", async () => {
    const { result } = renderHook(() => useUpdater(fake()));
    await waitFor(() => expect(result.current.version).toBe("0.4.0"));
    expect(result.current.state).toBe("idle");
  });

  it("ohne neue Version: aktuell", async () => {
    const { result } = renderHook(() => useUpdater(fake()));
    await act(async () => result.current.check());
    expect(result.current.state).toBe("uptodate");
  });

  it("mit neuer Version: verfügbar mit Info", async () => {
    const { result } = renderHook(() => useUpdater(fake({ check: async () => found })));
    await act(async () => result.current.check());
    expect(result.current.state).toBe("available");
    expect(result.current.info).toEqual(found);
  });

  it("Fehler beim Suchen werden gemeldet, nie geworfen", async () => {
    const api = fake({ check: async () => Promise.reject(new Error("kein Netz")) });
    const { result } = renderHook(() => useUpdater(api));
    await act(async () => result.current.check());
    expect(result.current.state).toBe("error");
    expect(result.current.error).toBe("kein Netz");
  });

  it("Installieren meldet den Fortschritt und endet bereit", async () => {
    const api = fake({
      check: async () => found,
      install: async (onProgress) => {
        onProgress(50, 200);
        onProgress(200, 200);
      },
    });
    const { result } = renderHook(() => useUpdater(api));
    await act(async () => result.current.check());
    await act(async () => result.current.install());
    expect(result.current.state).toBe("ready");
    expect(result.current.progress).toBe(1);
  });

  it("Die Rückfrage nach ungespeicherten Entwürfen kann das Installieren anhalten", async () => {
    const install = vi.fn(async () => undefined);
    const { result } = renderHook(() => useUpdater(fake({ check: async () => found, install })));
    await act(async () => result.current.check());
    await act(async () => result.current.install(async () => false));
    expect(install).not.toHaveBeenCalled();
    expect(result.current.state).toBe("available");
  });

  it("Neustart nur im Zustand bereit", async () => {
    const api = fake({ check: async () => found });
    const { result } = renderHook(() => useUpdater(api));
    await act(async () => result.current.restart());
    expect(api.restart).not.toHaveBeenCalled();
    await act(async () => result.current.check());
    await act(async () => result.current.install());
    await act(async () => result.current.restart());
    expect(api.restart).toHaveBeenCalledTimes(1);
  });

  it("Fehler beim Installieren führen in den Zustand Fehler", async () => {
    const api = fake({
      check: async () => found,
      install: async () => Promise.reject(new Error("Signatur ungültig")),
    });
    const { result } = renderHook(() => useUpdater(api));
    await act(async () => result.current.check());
    await act(async () => result.current.install());
    expect(result.current.state).toBe("error");
    expect(result.current.error).toBe("Signatur ungültig");
  });
});
