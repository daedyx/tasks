import { renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { writeKeepRunning } from "./backgroundPrefs";
import type { BackgroundApi, CloseEvent } from "./backgroundApi";
import { useBackgroundRun } from "./useBackgroundRun";

function fake() {
  let onClose: ((e: CloseEvent) => void) | undefined;
  const api: BackgroundApi = {
    setupTray: vi.fn(async () => () => undefined),
    onCloseRequested: vi.fn(async (h) => {
      onClose = h;
      return () => undefined;
    }),
    hideWindow: vi.fn(async () => undefined),
    showWindow: vi.fn(async () => undefined),
    quit: vi.fn(async () => undefined),
    autostart: { isEnabled: async () => false, set: async () => undefined },
  };
  return { api, close: (e: CloseEvent) => onClose?.(e) };
}

describe("useBackgroundRun", () => {
  beforeEach(() => localStorage.clear());

  it("legt das Symbol im Infobereich an, mit den Beschriftungen aus de.json", async () => {
    const { api } = fake();
    renderHook(() => useBackgroundRun(api));
    await waitFor(() => expect(api.setupTray).toHaveBeenCalled());
    const arg = vi.mocked(api.setupTray).mock.calls[0][0];
    expect(arg.openLabel).toBe("Daedyx Tasks öffnen");
    expect(arg.quitLabel).toBe("Beenden");
    expect(arg.tooltip).toBe("Daedyx Tasks");
  });

  it("Schließen versteckt das Fenster statt zu beenden, solange die Einstellung an ist", async () => {
    const { api, close } = fake();
    renderHook(() => useBackgroundRun(api));
    await waitFor(() => expect(api.onCloseRequested).toHaveBeenCalled());
    const preventDefault = vi.fn();
    close({ preventDefault });
    expect(preventDefault).toHaveBeenCalled();
    expect(api.hideWindow).toHaveBeenCalled();
  });

  it("Schließen beendet normal, wenn die Einstellung aus ist (Wert wird erst beim Schließen gelesen)", async () => {
    const { api, close } = fake();
    renderHook(() => useBackgroundRun(api));
    await waitFor(() => expect(api.onCloseRequested).toHaveBeenCalled());
    writeKeepRunning(false);
    const preventDefault = vi.fn();
    close({ preventDefault });
    expect(preventDefault).not.toHaveBeenCalled();
    expect(api.hideWindow).not.toHaveBeenCalled();
  });

  it("Öffnen und Beenden aus dem Menü laufen über die Schnittstelle", async () => {
    const { api } = fake();
    renderHook(() => useBackgroundRun(api));
    await waitFor(() => expect(api.setupTray).toHaveBeenCalled());
    const arg = vi.mocked(api.setupTray).mock.calls[0][0];
    arg.onOpen();
    arg.onQuit();
    expect(api.showWindow).toHaveBeenCalled();
    expect(api.quit).toHaveBeenCalled();
  });

  it("räumt beim Abbau auf", async () => {
    const offTray = vi.fn();
    const offClose = vi.fn();
    const { api } = fake();
    api.setupTray = vi.fn(async () => offTray);
    api.onCloseRequested = vi.fn(async () => offClose);
    const { unmount } = renderHook(() => useBackgroundRun(api));
    await waitFor(() => expect(api.onCloseRequested).toHaveBeenCalled());
    await waitFor(() => expect(api.setupTray).toHaveBeenCalled());
    unmount();
    await waitFor(() => expect(offTray).toHaveBeenCalled());
    expect(offClose).toHaveBeenCalled();
  });
});
