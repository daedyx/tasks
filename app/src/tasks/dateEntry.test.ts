import { describe, expect, it } from "vitest";
import { parseQuickEntry } from "./dateEntry";

// Mittwoch, 11.11.2026, 10:00
const NOW = new Date(2026, 10, 11, 10, 0);
const at = (m: number, d: number, h = 0, mi = 0) => new Date(2026, m - 1, d, h, mi);

describe("parseQuickEntry", () => {
  it("Zahnarzt morgen 15 Uhr", () => {
    expect(parseQuickEntry("Zahnarzt morgen 15 Uhr", NOW)).toMatchObject({ title: "Zahnarzt", start: at(11, 12, 15), end: at(11, 12, 16), allDay: false });
  });

  it("wiederkehrend: Training jeden Montag 18:30", () => {
    const r = parseQuickEntry("Training jeden Montag 18:30", NOW)!;
    expect([r.title, r.rrule, r.start, r.end]).toEqual(["Training", "FREQ=WEEKLY;BYDAY=MO", at(11, 16, 18, 30), at(11, 16, 19, 30)]);
  });

  it("ganztägig mit Datum", () => {
    const r = parseQuickEntry("Fortbildung am 20.11. ganztägig", NOW)!;
    expect([r.title, r.allDay, r.start, r.end]).toEqual(["Fortbildung", true, at(11, 20), at(11, 21)]);
  });

  it("Zeitraum von 14 bis 16 Uhr", () => {
    const r = parseQuickEntry("Besprechung heute von 14 bis 16 Uhr", NOW)!;
    expect([r.title, r.start, r.end]).toEqual(["Besprechung", at(11, 11, 14), at(11, 11, 16)]);
  });

  it("Ort nach 'bei' und 'in'", () => {
    const r = parseQuickEntry("Mittagessen übermorgen 12:00 bei Anna", NOW)!;
    expect([r.title, r.location, r.start]).toEqual(["Mittagessen", "Anna", at(11, 13, 12)]);
    expect(parseQuickEntry("Sitzung morgen 9 Uhr in Raum 5", NOW)).toMatchObject({ title: "Sitzung", location: "Raum 5" });
  });

  it("Monatsname, Uhrzeit und Dauer", () => {
    const r = parseQuickEntry("Meeting am 5. Dezember um 9 Uhr für 2 Stunden", NOW)!;
    expect([r.title, r.start, r.end]).toEqual(["Meeting", at(12, 5, 9), at(12, 5, 11)]);
    expect(parseQuickEntry("Arzt morgen 10 Uhr 30 Minuten", NOW)).toMatchObject({ end: at(11, 12, 10, 30) });
  });

  it("Wochentage: der nächste, nie heute", () => {
    expect(parseQuickEntry("Arzt Freitag 8:15", NOW)).toMatchObject({ start: at(11, 13, 8, 15) });
    expect(parseQuickEntry("Arzt Mittwoch 8:15", NOW)).toMatchObject({ start: at(11, 18, 8, 15) });
    expect(parseQuickEntry("Arzt nächsten Montag 8 Uhr", NOW)).toMatchObject({ start: at(11, 16, 8) });
  });

  it("Datum ohne Jahr, das schon vorbei ist, gilt fürs nächste Jahr", () => {
    expect(parseQuickEntry("Geburtstag am 3.2.", NOW)).toMatchObject({ allDay: true, start: new Date(2027, 1, 3) });
    expect(parseQuickEntry("Konzert am 24.12.2027 20 Uhr", NOW)).toMatchObject({ start: new Date(2027, 11, 24, 20) });
  });

  it("Wiederholungen: täglich, wöchentlich, monatlich, jährlich", () => {
    expect(parseQuickEntry("Tabletten täglich 8 Uhr", NOW)).toMatchObject({ rrule: "FREQ=DAILY", start: at(11, 11, 8) });
    expect(parseQuickEntry("Putzen jede Woche", NOW)?.rrule).toBe("FREQ=WEEKLY");
    expect(parseQuickEntry("Miete monatlich am 1.12.", NOW)?.rrule).toBe("FREQ=MONTHLY");
    expect(parseQuickEntry("Urlaub jährlich", NOW)?.rrule).toBe("FREQ=YEARLY");
  });

  it("nur ein Titel wird ein ganztägiger Termin heute; Ganztägig lässt die Uhrzeit weg", () => {
    expect(parseQuickEntry("nur ein Titel", NOW)).toMatchObject({ title: "nur ein Titel", allDay: true, start: at(11, 11), end: at(11, 12) });
    expect(parseQuickEntry("Zahnarzt 15 Uhr", NOW)).toMatchObject({ allDay: false, start: at(11, 11, 15) });
  });

  it("merkt, ob der Text einen Zeitpunkt nannte", () => {
    expect(parseQuickEntry("Statusbericht", NOW)?.hasWhen).toBe(false);
    expect(parseQuickEntry("Zahnarzt morgen", NOW)?.hasWhen).toBe(true);
    expect(parseQuickEntry("Zahnarzt 15 Uhr", NOW)?.hasWhen).toBe(true);
    expect(parseQuickEntry("Putzen jede Woche", NOW)?.hasWhen).toBe(true);
  });

  it("liefert null bei leerer Eingabe, fehlendem Titel und ungültigen Daten", () => {
    expect(parseQuickEntry("", NOW)).toBeNull();
    expect(parseQuickEntry("   ", NOW)).toBeNull();
    expect(parseQuickEntry("morgen 15 Uhr", NOW)).toBeNull();
    expect(parseQuickEntry("Test am 31.02.", NOW)).toBeNull();
    expect(parseQuickEntry("Test um 25 Uhr", NOW)).toBeNull();
  });
});
