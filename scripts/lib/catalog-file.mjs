// Lokaler Katalog für den Daedyx Launcher: Ordner mit catalog.json, catalog.json.minisig und packages/.
// Format und Schema: D:\Programmierung\daedyx-launcher\crates\daedyx-update\src\catalog.rs (Schema 1).
import { copyFileSync, existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { buildCatalogRelease, sha256Hex } from "./release-manifest.mjs";
import { signText } from "./minisign.mjs";

const SCHEMA = 1;
const MIN_LAUNCHER = "0.1.0";

function versionKey(v) {
  return v.split("-")[0].split(".").map(Number);
}

function compareVersions(a, b) {
  const [x, y] = [versionKey(a), versionKey(b)];
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] - y[i];
  return 0;
}

function readCatalog(path) {
  if (!existsSync(path)) return { schema: SCHEMA, minLauncherVersion: MIN_LAUNCHER, apps: [] };
  try {
    const cat = JSON.parse(readFileSync(path, "utf8"));
    if (cat.schema !== SCHEMA || !Array.isArray(cat.apps)) throw new Error("Schema");
    return cat;
  } catch {
    throw new Error(`Katalog ${path} ist beschädigt oder hat ein unbekanntes Schema; nichts geändert`);
  }
}

/**
 * Fügt dem lokalen Katalog eine Version von `app` hinzu (gleiche Version wird ersetzt), kopiert das Setup nach
 * `packages/` und signiert den Katalog neu. Es wird erst geschrieben, wenn alles geprüft ist.
 */
export function updateLocalCatalog({ catalogDir, secretKeyText, app, version, date, notes, setupPath, uploadName }) {
  const catalogPath = join(catalogDir, "catalog.json");
  const catalog = readCatalog(catalogPath);

  const bytes = readFileSync(setupPath);
  const release = buildCatalogRelease(
    { version, date, notes, url: `packages/${uploadName}`, sha256: sha256Hex(bytes), size: statSync(setupPath).size },
    { allowRelative: true },
  );

  let entry = catalog.apps.find((a) => a.id === app.id);
  if (!entry) {
    entry = { ...app, releases: [] };
    catalog.apps.push(entry);
  } else {
    Object.assign(entry, app);
  }
  entry.releases = entry.releases.filter((r) => r.version !== version).concat(release);
  entry.releases.sort((a, b) => compareVersions(a.version, b.version));

  const json = `${JSON.stringify(catalog, null, 2)}\n`;
  const signature = signText(json, secretKeyText, `Daedyx Katalog, ${app.id} ${version}`);

  mkdirSync(join(catalogDir, "packages"), { recursive: true });
  copyFileSync(setupPath, join(catalogDir, "packages", uploadName));
  // Katalog und Signatur zuletzt und über Zwischendateien, damit ein Abbruch keinen halben Zustand hinterlässt
  writeFileSync(`${catalogPath}.tmp`, json);
  writeFileSync(`${catalogPath}.minisig.tmp`, signature);
  renameSync(`${catalogPath}.tmp`, catalogPath);
  renameSync(`${catalogPath}.minisig.tmp`, `${catalogPath}.minisig`);
  return { catalogPath, release };
}
