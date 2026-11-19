import { describe, expect, it } from "vitest";
import {
  DARK_PRESETS,
  DEFAULT_DARK_STYLE,
  applyDarkStyle,
  darkMix,
  darkVars,
  normalizeStyle,
  parseDarkStyle,
  presetSwatch,
} from "./darkStyle";

describe("darkMix", () => {
  it("lässt die Mitte unverändert", () => {
    expect(darkMix(50)).toEqual({ black: 0, white: 0 });
  });

  it("mischt links Schwarz bei (bis 80 %), rechts Weiß (bis 20 %)", () => {
    expect(darkMix(0)).toEqual({ black: 80, white: 0 });
    expect(darkMix(25)).toEqual({ black: 40, white: 0 });
    expect(darkMix(75)).toEqual({ black: 0, white: 10 });
    expect(darkMix(100)).toEqual({ black: 0, white: 20 });
  });
});

describe("normalizeStyle", () => {
  it("begrenzt, rundet und ergänzt fehlende Felder mit dem Standard", () => {
    expect(normalizeStyle({})).toEqual(DEFAULT_DARK_STYLE);
    expect(normalizeStyle({ level: 130, hue: 400, tint: -5 })).toEqual({ level: 100, hue: 40, tint: 0 });
    expect(normalizeStyle({ level: 33.6, hue: -10 })).toEqual({ level: 34, hue: 350, tint: 30 });
    expect(normalizeStyle({ level: Number.NaN })).toEqual(DEFAULT_DARK_STYLE);
  });
});

describe("darkVars / applyDarkStyle", () => {
  it("rechnet die Stufen in Variablen um (Tönung 30 = Faktor 1)", () => {
    expect(darkVars({ level: 30, hue: 200, tint: 60 })).toEqual({
      "--dark-black": "32%",
      "--dark-white": "0%",
      "--dark-hue": "200",
      "--dark-tint": "2",
    });
  });

  it("setzt die Variablen auf dem Element", () => {
    const el = document.createElement("div");
    applyDarkStyle({ level: 90, hue: 10, tint: 0 }, el);
    expect(el.style.getPropertyValue("--dark-white")).toBe("16%");
    expect(el.style.getPropertyValue("--dark-hue")).toBe("10");
    expect(el.style.getPropertyValue("--dark-tint")).toBe("0");
  });
});

describe("parseDarkStyle", () => {
  it("liefert den Standard bei fehlendem oder kaputtem Wert", () => {
    expect(parseDarkStyle(null)).toEqual(DEFAULT_DARK_STYLE);
    expect(parseDarkStyle("{kaputt")).toEqual(DEFAULT_DARK_STYLE);
    expect(parseDarkStyle("null")).toEqual(DEFAULT_DARK_STYLE);
  });

  it("übernimmt gültige Werte und begrenzt ungültige", () => {
    expect(parseDarkStyle(JSON.stringify({ level: 20, hue: 140, tint: 500 }))).toEqual({ level: 20, hue: 140, tint: 100 });
    expect(parseDarkStyle(JSON.stringify({ level: "x" }))).toEqual(DEFAULT_DARK_STYLE);
  });
});

describe("Vorlagen", () => {
  it("enthalten Neutralgrau und den heutigen Standard", () => {
    const neutral = DARK_PRESETS.find((p) => p.id === "neutral")!;
    expect(neutral.tint).toBe(0);
    const blue = DARK_PRESETS.find((p) => p.id === "blue")!;
    expect({ hue: blue.hue, tint: blue.tint }).toEqual({ hue: DEFAULT_DARK_STYLE.hue, tint: DEFAULT_DARK_STYLE.tint });
  });

  it("liefern ein hsl-Muster", () => {
    expect(presetSwatch({ hue: 220, tint: 30 })).toBe("hsl(220 14% 22%)");
    expect(presetSwatch({ hue: 220, tint: 0 })).toBe("hsl(220 0% 22%)");
  });
});
