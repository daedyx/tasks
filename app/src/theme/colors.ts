/** Die benannten Palettenfarben aus `styles/tokens.css`. Frei wählbare Farben kommen mit Paket K5. */
export const COLOR_NAMES = ["blue", "teal", "green", "amber", "red", "pink", "purple", "slate"] as const;

export type ColorName = (typeof COLOR_NAMES)[number];
