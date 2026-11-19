import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { COLOR_NAMES } from "../theme/colors";
import { contrastRatio, evalColor, themeVars, type Vars } from "./colorEval.testkit";

const css = readFileSync(join(__dirname, "tokens.css"), "utf8");
const themes = themeVars(css);

const SURFACES = ["--surface-rail", "--surface-nav", "--surface-list", "--surface-reading", "--surface-toolbar"];
const THEMED = [...SURFACES, "--border-strong", "--tint-chrome", "--selected-mix", "--scrim", "--shadow-float", "--mail-paper"];

/** Dunkelmodus-Stil: Standard, hellster Regler, dunkelster Regler, warm und kräftig getönt */
const DARK_STYLES: Vars[] = [{}, { "--dark-white": "10%" }, { "--dark-black": "80%" }, { "--dark-hue": "30", "--dark-tint": "3.3" }];

const ratio = (vars: Vars, fg: string, bg: string) => contrastRatio(evalColor(`var(${fg})`, vars), evalColor(`var(${bg})`, vars));
const lum = (vars: Vars, token: string) => contrastRatio(evalColor(`var(${token})`, vars), [0, 0, 0]);

describe("Flächen-Tokens", () => {
  it("jedes Flächen-Token steht genau dreimal: hell, dunkel per Windows, dunkel per Umschalter", () => {
    for (const token of THEMED) {
      expect(css.split(`${token}:`).length - 1, token).toBe(3);
    }
  });

  it("beide dunklen Blöcke tragen dieselben Werte", () => {
    for (const token of THEMED) {
      const values = [...css.matchAll(new RegExp(`${token}:\\s*([^;]+);`, "g"))].map((m) => m[1].trim());
      expect(values[1], token).toBe(values[2]);
    }
  });

  it("Hellmodus: Bereichsleiste und Ordnerbaum sind getönt, die Liste ist am hellsten, die Kopfzeile hebt sich von der Lesefläche ab", () => {
    const v = themes.light;
    expect(lum(v, "--surface-list")).toBeGreaterThanOrEqual(lum(v, "--surface-nav"));
    expect(lum(v, "--surface-nav")).toBeGreaterThan(lum(v, "--surface-rail"));
    expect(lum(v, "--surface-reading")).toBeGreaterThan(lum(v, "--surface-toolbar"));
  });

  it("Dunkelmodus: Leiste am dunkelsten, dann Ordnerbaum, Liste, Lesefläche", () => {
    for (const v of [themes.osDark, themes.dark]) {
      expect(lum(v, "--surface-rail")).toBeLessThan(lum(v, "--surface-nav"));
      expect(lum(v, "--surface-nav")).toBeLessThan(lum(v, "--surface-list"));
      expect(lum(v, "--surface-list")).toBeLessThan(lum(v, "--surface-reading"));
    }
  });

  it("Text und gedämpfter Text bleiben auf allen Flächen lesbar (≥ 4,5:1), hell und in allen Dunkel-Stilen", () => {
    const sets: [string, Vars][] = [["hell", themes.light]];
    DARK_STYLES.forEach((style, i) => {
      sets.push([`dunkel (Windows), Stil ${i}`, { ...themes.osDark, ...style }], [`dunkel (Umschalter), Stil ${i}`, { ...themes.dark, ...style }]);
    });
    for (const [name, vars] of sets) {
      for (const surface of [...SURFACES, "--bg-hover", "--bg-surface", "--bg-subtle"]) {
        for (const fg of ["--text", "--text-muted"]) {
          expect(ratio(vars, fg, surface), `${fg} auf ${surface}, ${name}`).toBeGreaterThanOrEqual(4.5);
        }
      }
    }
  });

  it("gesperrte Knöpfe: Schrift auf ihrer Fläche und auf Leisten und Listen lesbar", () => {
    for (const vars of [themes.light, themes.osDark, themes.dark]) {
      expect(ratio(vars, "--btn-off-text", "--btn-off-bg")).toBeGreaterThanOrEqual(4.5);
      expect(ratio(vars, "--btn-off-text", "--surface-toolbar")).toBeGreaterThanOrEqual(4.5);
      expect(ratio(vars, "--btn-off-text", "--surface-list")).toBeGreaterThanOrEqual(4.5);
    }
  });

  it("Hauptknopf mit Akzentfläche: Schrift darauf lesbar (arbeitend: Sättigung leicht reduziert)", () => {
    for (const vars of [themes.light, themes.dark]) {
      for (const color of COLOR_NAMES) {
        expect(ratio(vars, `--c-${color}-on`, `--c-${color}`), color).toBeGreaterThanOrEqual(4.5);
      }
    }
  });

  it("kräftige Auswahl (Akzent über der Liste) trägt Text mit ≥ 4,5:1 in allen acht Bereichsfarben", () => {
    for (const [name, vars] of [["hell", themes.light], ["dunkel", themes.dark]] as const) {
      const mix = parseFloat(vars["--selected-mix"]);
      for (const color of COLOR_NAMES) {
        const strong = evalColor(`color-mix(in srgb, var(--c-${color}) ${mix}%, var(--surface-list))`, vars);
        expect(contrastRatio(evalColor("var(--text)", vars), strong), `${color} ${name}`).toBeGreaterThanOrEqual(4.5);
      }
    }
  });

  it("Bereichsleiste, Kopfleiste und Schreibfenster-Kopf (Akzent-getönte Leiste) tragen Text und gedämpften Text in allen Bereichsfarben", () => {
    for (const [name, vars] of [["hell", themes.light], ["dunkel", themes.dark]] as const) {
      const tint = parseFloat(vars["--tint-chrome"]);
      for (const color of COLOR_NAMES) {
        const chrome = evalColor(`color-mix(in srgb, var(--c-${color}) ${tint}%, var(--surface-rail))`, vars);
        for (const fg of ["--text", "--text-muted"]) {
          expect(contrastRatio(evalColor(`var(${fg})`, vars), chrome), `${fg} auf Leiste ${color} ${name}`).toBeGreaterThanOrEqual(4.5);
        }
      }
    }
  });

  it("Mail-Papier ist ein warmes Hell/Grau (nicht reinweiß) und trägt die Mailschrift mit hohem Kontrast", () => {
    for (const vars of [themes.light, themes.osDark, themes.dark]) {
      expect(ratio(vars, "--mail-ink", "--mail-paper")).toBeGreaterThanOrEqual(12);
      const paper = evalColor("var(--mail-paper)", vars);
      expect(paper[0]).toBeGreaterThan(paper[2]);
      expect(Math.min(...paper)).toBeLessThan(252);
    }
  });
});
