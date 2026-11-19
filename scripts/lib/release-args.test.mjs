import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { parseCli } from "./release-args.mjs";

describe("parseCli", () => {
  it("liest Optionen mit Wert", () => {
    const o = parseCli(["--dir", "a", "--version", "0.5.0", "--base-url", "https://x/", "--out", "o", "--date", "2026-11-15"]);
    expect(o).toMatchObject({ dir: "a", version: "0.5.0", baseUrl: "https://x/", out: "o", date: "2026-11-15", notes: "" });
  });

  it("liest die Notizen aus einer Datei, auch mit Anführungszeichen, Zeilenumbrüchen und führendem --", () => {
    const f = join(mkdtempSync(join(tmpdir(), "daedyx-notes-")), "notes.txt");
    const text = '--neu: "Kalender"\nzweite Zeile';
    writeFileSync(f, text, "utf8");
    expect(parseCli(["--version", "0.5.0", "--notes-file", f]).notes).toBe(text);
  });

  it("schaltet allow-local-http nur mit dem Wort true ein", () => {
    expect(parseCli(["--allow-local-http", "true"]).allowLocalHttp).toBe(true);
    expect(parseCli(["--allow-local-http", "false"]).allowLocalHttp).toBe(false);
    expect(parseCli([]).allowLocalHttp).toBe(false);
  });

  it("lehnt unbekannte Optionen ab", () => {
    expect(() => parseCli(["--unbekannt", "x"])).toThrow();
  });

  it("nimmt den Katalogordner und den Schlüssel", () => {
    const o = parseCli(["--catalog-dir", "c", "--catalog-key", "k"]);
    expect(o).toMatchObject({ catalogDir: "c", catalogKey: "k" });
  });
});
