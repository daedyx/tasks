import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import App from "./App";
import { fakeTasks } from "./tasks/fakeTasks";

beforeEach(() => {
  localStorage.clear();
  fakeTasks.reset();
});

describe("Daedyx Tasks (Hauptfenster)", () => {
  it("startet direkt im Aufgabenbereich", async () => {
    render(<App />);
    expect(screen.getByRole("heading", { level: 1, name: "Daedyx Tasks" })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Heute" })).toBeInTheDocument();
    expect(screen.getByRole("navigation", { name: "Aufgabenlisten" })).toBeInTheDocument();
  });

  it("legt eine Liste an, schreibt eine Aufgabe und hakt sie ab", async () => {
    render(<App />);
    fireEvent.click(await screen.findByRole("button", { name: /Neue Liste/ }));
    const name = screen.getByRole("textbox", { name: "Name der neuen Liste" });
    fireEvent.change(name, { target: { value: "Urlaub" } });
    fireEvent.keyDown(name, { key: "Enter" });
    await screen.findByRole("heading", { name: "Urlaub" });
    const entry = screen.getByRole("textbox", { name: "Neue Aufgabe" });
    fireEvent.change(entry, { target: { value: "Koffer packen" } });
    fireEvent.click(screen.getByRole("button", { name: "Hinzufügen" }));
    fireEvent.click(await screen.findByRole("checkbox", { name: "Koffer packen abhaken" }));
    expect(await screen.findByRole("checkbox", { name: "Koffer packen wieder öffnen" })).toBeChecked();
  });

  it("öffnet und schließt die Einstellungen (Darstellung und Updates)", async () => {
    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Einstellungen öffnen" }));
    expect(screen.getByRole("heading", { name: "Einstellungen" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Über Daedyx Tasks" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Einstellungen öffnen" }));
    expect(await screen.findByRole("navigation", { name: "Aufgabenlisten" })).toBeInTheDocument();
  });

  it("wechselt das Design und merkt es sich", () => {
    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Design: wie Windows" }));
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
    expect(localStorage.getItem("daedyx.theme")).toBe("light");
  });
});
