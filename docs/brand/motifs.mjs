// Symbolfamilie Daedyx (Richtung E „Farbe und Motiv“): eigene Zeichnung aus Code, gleiches Raster wie die
// Launcher-Marke (72 x 72, Kachel von 4 bis 68, Rundung 14). Jede App hat dieselbe Kachelform,
// aber eigene Farbe und eigenes Motiv.
// Urheberschaft: Daedyx contributors, Lizenz wie das Projekt (MIT).
export const COLORS = { indigo: '#34337F', amber: '#E8A33D', white: '#FFFFFF', petrol: '#1F7F8E' };
const { indigo, amber, white, petrol } = COLORS;
const stroke = (w, c) => `fill="none" stroke="${c}" stroke-width="${w}" stroke-linecap="round" stroke-linejoin="round"`;

const APPS = {
  // Abgerundetes Quadrat wie der Launcher; Postfach mit Kalenderringen
  desk: {
    shape: `<rect x="4" y="4" width="64" height="64" rx="14" fill="${indigo}"/>`,
    motif: `<rect x="14" y="23" width="44" height="31" rx="5" fill="${white}"/>`
      + `<path d="M16 27L36 42L56 27" ${stroke(3.5, indigo)}/>`
      + `<path d="M26 14V25M46 14V25" ${stroke(4.5, amber)}/>`,
  },
  // Bernstein; ein einziger großer Haken
  tasks: {
    shape: `<rect x="4" y="4" width="64" height="64" rx="14" fill="${amber}"/>`,
    motif: `<path d="M19 37L31 49L54 22" ${stroke(9, indigo)}/>`,
  },
  // Petrol; zwei schräge Karten und ein Markerstrich
  board: {
    shape: `<rect x="4" y="4" width="64" height="64" rx="14" fill="${petrol}"/>`,
    motif: `<g transform="rotate(-9 29 31)"><rect x="15" y="19" width="22" height="22" rx="3" fill="${amber}"/></g>`
      + `<g transform="rotate(7 44 43)"><rect x="33" y="31" width="23" height="22" rx="3" fill="${white}"/></g>`
      + `<path d="M20 56Q28 49 34 55T50 54" ${stroke(3, amber)}/>`,
  },
};
export const APP_NAMES = Object.keys(APPS);

/** Vollständiges SVG-Dokument; die Fläche füllt das Bild, damit `tauri icon` sie 1:1 skaliert. */
export function iconSvg(app) {
  if (!APPS[app]) throw new Error(`Unbekannte App: ${app}`);
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="4 4 64 64" width="1024" height="1024">${APPS[app].shape}${APPS[app].motif}</svg>\n`;
}
