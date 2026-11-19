export type DarkStyle = { level: number; hue: number; tint: number };

export const DEFAULT_DARK_STYLE: DarkStyle = { level: 50, hue: 220, tint: 30 };

// Tönung 30 entspricht der heutigen Sättigung (Faktor 1); 0 ist neutral grau, 100 etwa dreifach
const TINT_UNIT = 30;

export const DARK_PRESETS = [
  { id: "neutral", labelKey: "settings.display.preset_neutral", hue: 220, tint: 0 },
  { id: "blue", labelKey: "settings.display.preset_blue", hue: 220, tint: 30 },
  { id: "teal", labelKey: "settings.display.preset_teal", hue: 185, tint: 40 },
  { id: "green", labelKey: "settings.display.preset_green", hue: 140, tint: 35 },
  { id: "purple", labelKey: "settings.display.preset_purple", hue: 270, tint: 40 },
  { id: "warm", labelKey: "settings.display.preset_warm", hue: 30, tint: 35 },
] as const;

const num = (v: unknown, fallback: number): number =>
  typeof v === "number" && Number.isFinite(v) ? v : fallback;

export function normalizeStyle(s: Partial<DarkStyle>): DarkStyle {
  const clamp = (n: number) => Math.min(100, Math.max(0, Math.round(n)));
  return {
    level: clamp(num(s.level, DEFAULT_DARK_STYLE.level)),
    hue: ((Math.round(num(s.hue, DEFAULT_DARK_STYLE.hue)) % 360) + 360) % 360,
    tint: clamp(num(s.tint, DEFAULT_DARK_STYLE.tint)),
  };
}

// Links der Mitte wird Schwarz beigemischt (bis 80 %), rechts Weiß (bis 20 %); die Schrift wird mit aufgehellt
export function darkMix(level: number): { black: number; white: number } {
  return {
    black: level < 50 ? Math.round((50 - level) * 1.6) : 0,
    white: level > 50 ? Math.round((level - 50) * 0.4) : 0,
  };
}

export function darkVars(style: DarkStyle): Record<string, string> {
  const { black, white } = darkMix(style.level);
  return {
    "--dark-black": `${black}%`,
    "--dark-white": `${white}%`,
    "--dark-hue": String(style.hue),
    "--dark-tint": String(Math.round((style.tint / TINT_UNIT) * 100) / 100),
  };
}

export function applyDarkStyle(style: DarkStyle, root: HTMLElement = document.documentElement): void {
  for (const [name, value] of Object.entries(darkVars(style))) root.style.setProperty(name, value);
}

export function parseDarkStyle(raw: string | null): DarkStyle {
  try {
    const data = JSON.parse(raw ?? "");
    return normalizeStyle(data && typeof data === "object" ? data : {});
  } catch {
    return DEFAULT_DARK_STYLE;
  }
}

export function presetSwatch(p: { hue: number; tint: number }): string {
  return `hsl(${p.hue} ${Math.round((14 * p.tint) / TINT_UNIT)}% 22%)`;
}
