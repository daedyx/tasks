import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

// Schützt vor einem Fehler, der erst im Lauf auf GitHub auffiele: ein Pfad in release.ps1 mit Steuerzeichen
// (zum Beispiel ein Tabulator, weil beim Schreiben aus "\t" ein echtes Tabulatorzeichen wurde) oder ein Pfad, den es nicht gibt.
const root = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const text = readFileSync(join(root, "scripts", "release.ps1"), "utf8");

describe("release.ps1", () => {
  it("enthält in Zeichenketten keine Tabulatoren oder anderen Steuerzeichen", () => {
    const bad = [...text.matchAll(/"[^"\n]*[\x00-\x08\x09\x0b\x0c\x0e-\x1f][^"\n]*"/g)].map((m) => JSON.stringify(m[0]));
    expect(bad).toEqual([]);
  });

  it("verweist nur auf Skripte, die es gibt", () => {
    const refs = [...text.matchAll(/"(scripts[\\/][^"$]*\.(?:ps1|mjs))"/g)].map((m) => m[1]);
    expect(refs.length).toBeGreaterThan(0);
    const missing = refs.filter((r) => !existsSync(join(root, r.replaceAll("\\", "/"))));
    expect(missing).toEqual([]);
  });

  it("ruft die Probe der Freigabe-Regeln mit einem Pfad auf, den es gibt", () => {
    const m = text.match(/Join-Path \$root "([^"]*test-check-release-inputs\.ps1)"/);
    expect(m, "Aufruf der Probe fehlt in release.ps1").not.toBeNull();
    expect(existsSync(join(root, m[1].replaceAll("\\", "/")))).toBe(true);
  });
});
