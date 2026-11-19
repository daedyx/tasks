import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

function cssFiles(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const path = join(dir, e.name);
    if (e.isDirectory()) return cssFiles(path);
    return e.name.endsWith(".css") ? [path] : [];
  });
}

const buttons = readFileSync(join(__dirname, "buttons.css"), "utf8").replace(/\/\*[\s\S]*?\*\//g, "");

describe("Knopfzustände (zentral in ui/buttons.css)", () => {
  it("gesperrt: lesbare Schrift aus dem Token, kein Ausgrauen über opacity", () => {
    expect(buttons).toMatch(/button:disabled:not\(\[aria-busy="true"\]\)[^{]*\{[^}]*color:\s*var\(--btn-off-text\)/);
    expect(buttons).not.toMatch(/opacity/);
  });

  it("gesperrte Hauptknöpfe bekommen eine ruhige Fläche statt Akzent mit Schleier", () => {
    const rule = buttons.match(/\.wizard__primary:disabled:not\(\[aria-busy="true"\]\)[^{]*\{[^}]*\}/)?.[0] ?? "";
    expect(rule).toContain("var(--btn-off-bg)");
    expect(rule).toContain("var(--btn-off-text)");
    for (const cls of ["cbtn--primary", "cbtn--danger", "sigs__btn--primary", "wizard__primary", "account__primary"]) {
      expect(buttons, cls).toContain(`.${cls}:disabled:not([aria-busy="true"])`);
    }
  });

  it("arbeitend (aria-busy): Spinner, Fortschrittszeiger, Hauptknöpfe leicht entsättigt, Schrift unverändert", () => {
    expect(buttons).toMatch(/button\[aria-busy="true"\]::after[^{]*\{[^}]*animation:/);
    expect(buttons).toMatch(/button\[aria-busy="true"\][^{]*\{[^}]*cursor:\s*progress/);
    expect(buttons).toMatch(/\.wizard__primary\[aria-busy="true"\][^{]*\{[^}]*saturate\(/);
    expect(buttons).not.toMatch(/(?<!:not\()\[aria-busy="true"\][^{)]*\{[^}]*[{;\s](color|background):/);
  });

  it("Spinner steht still bei reduzierter Bewegung", () => {
    expect(buttons).toMatch(/prefers-reduced-motion:\s*reduce\)[\s\S]*animation:\s*none/);
  });

  it("kein anderes Stylesheet graut gesperrte Knöpfe über opacity aus", () => {
    const offenders: string[] = [];
    for (const file of cssFiles(join(__dirname, ".."))) {
      for (const [, selector, body] of readFileSync(file, "utf8").matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
        if (/:disabled|\[disabled\]|aria-busy/.test(selector) && /opacity/.test(body)) offenders.push(`${file}: ${selector.trim()}`);
      }
    }
    expect(offenders).toEqual([]);
  });
});
