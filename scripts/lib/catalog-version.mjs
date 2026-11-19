// Prüft vor dem Veröffentlichen, dass die neue Version höher ist als jede bisherige der App im Katalog.
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const VERSION = /^(\d+)\.(\d+)\.(\d+)$/;

function parts(v) {
  const m = VERSION.exec(v);
  if (!m) throw new Error(`Ungültige Version: ${v} (erwartet x.y.z)`);
  return m.slice(1).map(Number);
}

function compare(a, b) {
  const [x, y] = [parts(a), parts(b)];
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] - y[i];
  return 0;
}

/** `catalogJson` ist der Text des bisherigen Katalogs oder `null`, wenn es noch keinen gibt. */
export function assertNewer(catalogJson, appId, version) {
  parts(version);
  if (catalogJson === null) return;
  let catalog;
  try {
    catalog = JSON.parse(catalogJson);
  } catch {
    throw new Error("Der bisherige Katalog ist beschädigt; es wird nichts veröffentlicht");
  }
  const releases = (catalog.apps ?? []).find((a) => a.id === appId)?.releases ?? [];
  for (const r of releases) {
    if (compare(version, r.version) <= 0) {
      throw new Error(`Version ${version} muss höher sein als die schon veröffentlichte ${r.version}`);
    }
  }
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? "").href) {
  // node catalog-version.mjs <catalog.json oder -> <app> <version>
  const [file, app, version] = process.argv.slice(2);
  let text = null;
  if (file !== "-") text = readFileSync(file, "utf8");
  assertNewer(text, app, version);
  console.log(`Version ${version} ist neu für ${app}.`);
}
