// Symbolfamilie Daedyx (Richtung B „Fläche“): eigene Zeichnung aus Code, gleiches Raster wie die
// Launcher-Marke (72 x 72, Kachel von 4 bis 68, Rundung 14, Indigo und Bernstein).
// Urheberschaft: Daedyx contributors, Lizenz wie das Projekt (MIT).
export const COLORS = { indigo: '#34337F', amber: '#E8A33D', white: '#FFFFFF' };
const { indigo, amber, white } = COLORS;
const stroke = (w, c) => `fill="none" stroke="${c}" stroke-width="${w}" stroke-linecap="round" stroke-linejoin="round"`;

const MOTIFS = {
  // Postfach mit Kalenderringen; das V der Klappe zugleich als Haken gelesen
  desk: `<rect x="15" y="23" width="42" height="30" rx="5" fill="${white}"/>`
    + `<path d="M17 27L36 41L55 27" ${stroke(3.5, indigo)}/>`
    + `<path d="M26 15V25M46 15V25" ${stroke(4.5, amber)}/>`,
  // Abgehakte Liste
  tasks: `<circle cx="22.5" cy="23" r="6.5" fill="${amber}"/>`
    + `<path d="M19.5 23L21.7 25.3L25.8 20.5" ${stroke(2.6, indigo)}/>`
    + `<rect x="33" y="20" width="23" height="6" rx="3" fill="${white}"/>`
    + `<circle cx="22.5" cy="48" r="6.5" fill="${white}"/>`
    + `<rect x="33" y="45" width="17" height="6" rx="3" fill="${white}"/>`,
  // Zeichenfläche mit zwei verbundenen Karten
  board: `<rect x="14" y="17" width="44" height="38" rx="5" fill="${white}"/>`
    + `<rect x="20" y="23" width="15" height="10" rx="2" fill="${amber}"/>`
    + `<rect x="38" y="38" width="14" height="10" rx="2" fill="${indigo}"/>`
    + `<path d="M35 28H45V38" ${stroke(3, indigo)}/>`,
};
export const APP_NAMES = Object.keys(MOTIFS);

/** Vollständiges SVG-Dokument; Kachel füllt die Fläche, damit `tauri icon` sie 1:1 skaliert. */
export function iconSvg(app) {
  if (!MOTIFS[app]) throw new Error(`Unbekannte App: ${app}`);
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="4 4 64 64" width="1024" height="1024">`
    + `<rect x="4" y="4" width="64" height="64" rx="14" fill="${indigo}"/>${MOTIFS[app]}</svg>\n`;
}
