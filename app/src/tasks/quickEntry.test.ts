import { describe, expect, it } from "vitest";
import { parseQuickTask } from "./quickEntry";

// Mittwoch, 18.11.2026, 10:00 Ortszeit
const NOW = new Date(2026, 10, 18, 10, 0);

describe("Schnelleingabe Aufgaben", () => {
  it("nur ein Titel bleibt ein Titel", () => {
    expect(parseQuickTask("Müll rausbringen", NOW)).toEqual({ title: "Müll rausbringen", fields: { title: "Müll rausbringen" } });
  });

  it("leere Eingabe ergibt nichts", () => {
    expect(parseQuickTask("   ", NOW)).toBeNull();
    expect(parseQuickTask("!!", NOW)).toBeNull();
  });

  it("Ausrufezeichen setzen die Priorität und gehören nicht zum Titel", () => {
    expect(parseQuickTask("Steuer !!!", NOW)?.fields).toMatchObject({ title: "Steuer", priority: 3 });
    expect(parseQuickTask("! Steuer", NOW)?.fields).toMatchObject({ title: "Steuer", priority: 1 });
    expect(parseQuickTask("Hallo!", NOW)?.title).toBe("Hallo!");
  });

  it("ein Tag macht eine ganztägige Fälligkeit", () => {
    const q = parseQuickTask("Steuer morgen", NOW);
    expect(q?.fields).toMatchObject({ title: "Steuer", due: "2026-11-19", due_has_time: false });
  });

  it("Tag mit Uhrzeit macht eine Fälligkeit mit Uhrzeit (UTC-Sekunden)", () => {
    const q = parseQuickTask("Anruf morgen 15:30 !!", NOW);
    expect(q?.fields).toMatchObject({ title: "Anruf", due_has_time: true, priority: 2 });
    expect(new Date(Number(q?.fields.due) * 1000).getHours()).toBe(15);
  });
});
