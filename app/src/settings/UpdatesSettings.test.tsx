import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { UpdaterApi } from "../updates/updater";
import { UpdatesSettings } from "./UpdatesSettings";

function fake(over: Partial<UpdaterApi> = {}): UpdaterApi {
  return {
    currentVersion: async () => "0.4.0",
    check: async () => null,
    install: async () => undefined,
    restart: vi.fn(async () => undefined),
    ...over,
  };
}

describe("UpdatesSettings", () => {
  it("zeigt die Version und sucht beim Öffnen nach Updates", async () => {
    render(<UpdatesSettings api={fake()} />);
    expect(await screen.findByText("Installierte Version: 0.4.0")).toBeInTheDocument();
    expect(await screen.findByText("Du hast die neueste Version.")).toBeInTheDocument();
  });

  it("„Nach Updates suchen“ sucht erneut", async () => {
    const check = vi.fn(async () => null);
    render(<UpdatesSettings api={fake({ check })} />);
    await screen.findByText("Du hast die neueste Version.");
    fireEvent.click(screen.getByRole("button", { name: "Nach Updates suchen" }));
    await waitFor(() => expect(check).toHaveBeenCalledTimes(2));
  });

  it("zeigt eine neue Version mit Änderungen und installiert sie mit Fortschrittsbalken", async () => {
    let finish: () => void = () => undefined;
    const install = vi.fn(
      (onProgress: (d: number, t: number) => void) =>
        new Promise<void>((resolve) => {
          onProgress(50, 100);
          finish = resolve;
        }),
    );
    render(<UpdatesSettings api={fake({ check: async () => ({ version: "0.4.1", notes: "Mails schneller" }), install })} />);
    expect(await screen.findByText("Version 0.4.1 ist verfügbar.")).toBeInTheDocument();
    expect(screen.getByText("Mails schneller")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Jetzt installieren" }));
    const bar = await screen.findByRole("progressbar", { name: "Fortschritt" });
    expect(bar).toHaveAttribute("aria-valuenow", "50");
    finish();
    expect(await screen.findByText(/Das Update ist installiert/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Jetzt neu starten" }));
  });

  it("zeigt Fehler als Text", async () => {
    render(<UpdatesSettings api={fake({ check: async () => Promise.reject(new Error("kein Netz")) })} />);
    expect(await screen.findByText("Das Update ist fehlgeschlagen: kein Netz")).toBeInTheDocument();
  });

  it("die Einstellung „Beim Start nach Updates suchen“ ist standardmäßig an und lässt sich abschalten", async () => {
    localStorage.clear();
    render(<UpdatesSettings api={fake()} />);
    const box = await screen.findByRole("checkbox", { name: "Beim Start nach Updates suchen" });
    expect(box).toBeChecked();
    fireEvent.click(box);
    expect(box).not.toBeChecked();
    expect(localStorage.getItem("daedyx.updates.auto")).toBe("0");
  });
});
