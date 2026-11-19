import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { COLOR_NAMES } from "../theme/colors";

function files(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const path = join(dir, e.name);
    if (e.isDirectory()) return files(path);
    return /\.(css|tsx?)$/.test(e.name) && !/\.test\./.test(e.name) ? [path] : [];
  });
}

describe("Design-Tokens", () => {
  it("die dunklen Hintergründe sind in beiden dunklen Blöcken über Farbton, Tönung und Helligkeit einstellbar", () => {
    const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
    const count = (needle: string) => css.split(needle).length - 1;
    expect(count("var(--dark-hue")).toBe(10);
    expect(count("var(--dark-tint")).toBe(10);
    expect(count("var(--dark-black")).toBe(10);
    expect(count("var(--dark-white")).toBe(14);
  });

  it("keine Hex-Farben außerhalb von tokens.css", () => {
    const offenders = files(join(__dirname, ".."))
      .filter((f) => !f.endsWith("tokens.css"))
      .filter((f) => /#[0-9a-fA-F]{3,8}\b/.test(readFileSync(f, "utf8")));
    expect(offenders).toEqual([]);
  });

  it("dunkle Werte stehen für Windows-Einstellung und für den Umschalter", () => {
    const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
    expect(css).toContain('@media (prefers-color-scheme: dark)');
    expect(css).toContain(':root:not([data-theme="light"])');
    expect(css).toContain(':root[data-theme="dark"]');
  });

  it("color-scheme folgt dem Thema (native Bedienelemente)", () => {
    const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
    expect(css).toContain("color-scheme: light");
    expect(css).toContain("color-scheme: dark");
  });

  it("jede Palettenfarbe steht in allen drei Blöcken (hell, dunkel per Windows, dunkel per Umschalter)", () => {
    const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
    for (const name of COLOR_NAMES) {
      for (const suffix of ["", "-soft", "-on"]) {
        const count = css.split(`--c-${name}${suffix}:`).length - 1;
        expect(count, `--c-${name}${suffix}`).toBe(3);
      }
    }
  });

  it("Schrift auf Kalenderfarben und Schriftfarben stehen in den nötigen Blöcken", () => {
    const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
    for (const n of [1, 2, 3]) {
      expect(css.split(`--cal-${n}-on:`).length - 1, `--cal-${n}-on`).toBe(3);
    }
    expect(css).toContain("--ink-light:");
    expect(css).toContain("--ink-dark:");
  });
});
