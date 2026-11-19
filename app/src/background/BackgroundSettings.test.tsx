import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { BackgroundApi } from "./backgroundApi";
import { BackgroundSettings } from "./BackgroundSettings";

function fake(enabled = false) {
  const set = vi.fn(async () => undefined);
  const api = { autostart: { isEnabled: async () => enabled, set } } as unknown as BackgroundApi;
  return { api, set };
}

describe("BackgroundSettings", () => {
  beforeEach(() => localStorage.clear());

  it("„Im Infobereich weiterlaufen“ ist standardmäßig an und lässt sich abschalten", () => {
    render(<BackgroundSettings api={fake().api} />);
    const box = screen.getByRole("checkbox", { name: "Im Infobereich weiterlaufen, wenn das Fenster geschlossen wird" });
    expect(box).toBeChecked();
    fireEvent.click(box);
    expect(box).not.toBeChecked();
    expect(localStorage.getItem("daedyx.background.keepRunning")).toBe("0");
  });

  it("zeigt den Autostart-Zustand und schaltet ihn um", async () => {
    const { api, set } = fake(false);
    render(<BackgroundSettings api={api} />);
    const box = await screen.findByRole("checkbox", { name: "Mit Windows starten" });
    await waitFor(() => expect(box).not.toBeDisabled());
    expect(box).not.toBeChecked();
    fireEvent.click(box);
    await waitFor(() => expect(set).toHaveBeenCalledWith(true));
    expect(box).toBeChecked();
  });

  it("meldet einen Fehler beim Autostart als Text und lässt den Zustand unverändert", async () => {
    const { api, set } = fake(false);
    set.mockRejectedValueOnce(new Error("verweigert"));
    render(<BackgroundSettings api={api} />);
    const box = await screen.findByRole("checkbox", { name: "Mit Windows starten" });
    await waitFor(() => expect(box).not.toBeDisabled());
    fireEvent.click(box);
    expect(await screen.findByText("Der Autostart ließ sich nicht ändern: verweigert")).toBeInTheDocument();
    expect(box).not.toBeChecked();
  });

  it("ohne Tauri (Vorschau im Browser) ist der Autostart nicht schaltbar", async () => {
    render(<BackgroundSettings />);
    expect(await screen.findByRole("checkbox", { name: "Mit Windows starten" })).toBeDisabled();
  });
});
