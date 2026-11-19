// Schreibt die Quellvektoren: `node docs/brand/erzeuge-symbole.mjs` -> docs/brand/desk.svg, tasks.svg, board.svg
// Daraus erzeugt `tauri icon` die PNG-, ICO- und ICNS-Dateien (siehe README.md in diesem Ordner).
import { writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { APP_NAMES, iconSvg } from './motifs.mjs';

const dir = fileURLToPath(new URL('.', import.meta.url));
for (const app of APP_NAMES) {
  writeFileSync(`${dir}${app}.svg`, iconSvg(app));
  console.log(`geschrieben: ${app}.svg`);
}
