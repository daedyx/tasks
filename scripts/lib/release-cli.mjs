// Kommandozeile um die Module in scripts/lib: liest Setup (und Signatur) aus dem Bundle-Ordner und schreibt
//  - in den lokalen Katalog des Daedyx Launchers (catalogDir + catalogKey) und/oder
//  - latest.json und catalog-entry.json für eine Veröffentlichung unter baseUrl (out).
import { copyFileSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { updateLocalCatalog } from "./catalog-file.mjs";
import { parseCli } from "./release-args.mjs";
import { buildCatalogRelease, buildLatestJson, sha256Hex } from "./release-manifest.mjs";

export const TASKS_APP = {
  id: "tasks",
  name: "Daedyx Tasks",
  description: "Aufgaben mit Listen, Abschnitten, Board, Wiederholung und Erinnerung",
  license: "",
  author: "Daedyx",
  // Name des Uninstall-Schlüssels (Tauri-NSIS: Produktname); der Launcher erkennt darüber eine vorhandene Installation
  registryKey: "Daedyx Tasks",
};

function findSetup(dir, version) {
  const name = readdirSync(dir).find((f) => f.endsWith(`_${version}_x64-setup.exe`));
  if (!name) throw new Error(`Setup für Version ${version} fehlt in ${dir}`);
  return name;
}

export function run({ dir, out, version, notes = "", baseUrl, date, allowLocalHttp = false, catalogDir, catalogKey }) {
  if (!baseUrl && !catalogDir) throw new Error("Kein Ziel: --base-url (Veröffentlichung) oder --catalog-dir (lokaler Katalog) angeben");
  const name = findSetup(dir, version);
  const source = join(dir, name);
  // GitHub ersetzt Leerzeichen in Dateinamen durch Punkte; die Adresse muss zum hochgeladenen Namen passen.
  const uploadName = name.replaceAll(" ", ".");
  const result = {};

  if (baseUrl) {
    let signature;
    try {
      signature = readFileSync(`${source}.sig`, "utf8").trim();
    } catch {
      throw new Error(`Signatur fehlt: ${name}.sig (Build ohne TAURI_SIGNING_PRIVATE_KEY?)`);
    }
    const url = `${baseUrl.endsWith("/") ? baseUrl : `${baseUrl}/`}${uploadName}`;
    const opts = { allowLocalHttp };
    const release = buildCatalogRelease(
      { version, date, notes, url, sha256: sha256Hex(readFileSync(source)), size: statSync(source).size },
      opts,
    );
    const latest = buildLatestJson({ version, date, notes, url, signature }, opts);
    mkdirSync(out, { recursive: true });
    copyFileSync(source, join(out, uploadName));
    writeFileSync(join(out, "latest.json"), `${JSON.stringify(latest, null, 2)}\n`);
    writeFileSync(join(out, "catalog-entry.json"), `${JSON.stringify(release, null, 2)}\n`);
    Object.assign(result, { url, out });
  }

  if (catalogDir) {
    if (!catalogKey) throw new Error("Für den lokalen Katalog fehlt der Katalog-Schlüssel (--catalog-key)");
    const secretKeyText = readFileSync(catalogKey, "utf8");
    const { catalogPath } = updateLocalCatalog({
      catalogDir,
      secretKeyText,
      app: TASKS_APP,
      version,
      date,
      notes,
      setupPath: source,
      uploadName,
    });
    result.catalogPath = catalogPath;
  }
  return result;
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? "").href) {
  const result = run(parseCli(process.argv.slice(2)));
  if (result.out) console.log(`Veröffentlichung: ${result.out} (${result.url})`);
  if (result.catalogPath) console.log(`Lokaler Katalog aktualisiert: ${result.catalogPath}`);
}
