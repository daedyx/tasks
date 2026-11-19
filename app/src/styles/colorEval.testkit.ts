/**
 * Kleiner Auswerter für die Farbausdrücke in tokens.css (nur für Tests): Hex, hsl(), color-mix(in srgb, …), var(--x, Ersatz),
 * calc(a * b) mit Prozent. So lässt sich der Kontrast der echten Tokens prüfen, auch für die dunklen Werte aus --dark-*.
 */
export type Rgb = [number, number, number];
export type Vars = Record<string, string>;

const NAMED: Record<string, Rgb> = { black: [0, 0, 0], white: [255, 255, 255] };

/** Teilt an oberster Ebene (nicht in Klammern) an Leerraum oder Komma */
function split(text: string, sep: RegExp): string[] {
  const parts: string[] = [];
  let depth = 0;
  let cur = "";
  for (const ch of text.trim()) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (depth === 0 && sep.test(ch)) {
      if (cur.trim()) parts.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) parts.push(cur.trim());
  return parts;
}

function inner(expr: string): string {
  return expr.slice(expr.indexOf("(") + 1, expr.lastIndexOf(")"));
}

/** Wert einer var(): erst die Variable, sonst der Ersatzwert nach dem ersten Komma */
function lookup(expr: string, vars: Vars): string {
  const body = inner(expr);
  const comma = body.indexOf(",");
  const name = (comma < 0 ? body : body.slice(0, comma)).trim();
  const value = vars[name] ?? (comma < 0 ? "" : body.slice(comma + 1).trim());
  if (value === "") throw new Error(`Variable ohne Wert: ${name}`);
  return value;
}

/** Zahl oder Prozentwert (als Zahl, 15% wird 15) */
function num(expr: string, vars: Vars): number {
  const e = expr.trim();
  if (e.startsWith("var(")) return num(lookup(e, vars), vars);
  if (e.startsWith("calc(")) {
    const [a, op, b] = split(inner(e), /\s/);
    if (op !== "*") throw new Error(`calc: nur Multiplikation, nicht ${op}`);
    return num(a, vars) * num(b, vars);
  }
  return parseFloat(e);
}

export function hslToRgb(h: number, s: number, l: number): Rgb {
  const sat = Math.min(1, s / 100);
  const light = l / 100;
  const k = (n: number) => (n + h / 30) % 12;
  const a = sat * Math.min(light, 1 - light);
  const f = (n: number) => light - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
  return [f(0), f(8), f(4)].map((x) => x * 255) as Rgb;
}

export function evalColor(expr: string, vars: Vars): Rgb {
  const e = expr.trim();
  if (e.startsWith("var(")) return evalColor(lookup(e, vars), vars);
  if (e.startsWith("#")) {
    const n = parseInt(e.slice(1), 16);
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  }
  if (e in NAMED) return NAMED[e];
  if (e.startsWith("hsl(")) {
    const [h, s, l] = split(inner(e), /\s/);
    return hslToRgb(num(h, vars), num(s, vars), num(l, vars));
  }
  if (e.startsWith("color-mix(")) {
    const [, ...stops] = split(inner(e), /,/);
    const parsed = stops.map((stop) => {
      const words = split(stop, /\s/);
      const last = words[words.length - 1];
      const hasPct = words.length > 1 && (last.endsWith("%") || last.startsWith("calc(") || (last.startsWith("var(") && last.includes("%")));
      return { color: evalColor(hasPct ? words.slice(0, -1).join(" ") : stop, vars), pct: hasPct ? num(last, vars) : null };
    });
    const known = parsed.reduce((sum, p) => sum + (p.pct ?? 0), 0);
    const open = parsed.filter((p) => p.pct === null).length;
    const weights = parsed.map((p) => (p.pct ?? (100 - known) / open) / 100);
    const total = weights.reduce((a, b) => a + b, 0);
    return [0, 1, 2].map((i) => parsed.reduce((sum, p, j) => sum + p.color[i] * (weights[j] / total), 0)) as Rgb;
  }
  throw new Error(`Farbausdruck nicht unterstützt: ${e}`);
}

function luminance([r, g, b]: Rgb): number {
  const lin = (c: number) => {
    const s = Math.round(c) / 255;
    return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
}

export function contrastRatio(a: Rgb, b: Rgb): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

/** Liest die Deklarationen `--name: wert;` eines Blocks (Text zwischen { und }) */
export function declarations(block: string): Vars {
  const out: Vars = {};
  for (const m of block.matchAll(/(--[\w-]+)\s*:\s*([^;]+);/g)) out[m[1]] = m[2].trim();
  return out;
}

/**
 * Die drei Themen aus tokens.css: hell (die :root-Blöcke ohne Bedingung), dunkel per Windows-Einstellung, dunkel per Umschalter.
 * Die dunklen erben die hellen Werte, wo sie nichts überschreiben (so wie im Browser).
 */
export function themeVars(css: string): { light: Vars; osDark: Vars; dark: Vars } {
  const noComments = css.replace(/\/\*[\s\S]*?\*\//g, "");
  const blocks = [...noComments.matchAll(/([^{}]+)\{([^{}]*)\}/g)].map((m) => ({ head: m[1].trim(), vars: declarations(m[2]) }));
  const pick = (test: (head: string) => boolean) => Object.assign({}, ...blocks.filter((b) => test(b.head)).map((b) => b.vars));
  const light = pick((h) => h === ":root");
  return {
    light,
    osDark: { ...light, ...pick((h) => h.startsWith(':root:not([data-theme="light"])')) },
    dark: { ...light, ...pick((h) => h.startsWith(':root[data-theme="dark"]')) },
  };
}
