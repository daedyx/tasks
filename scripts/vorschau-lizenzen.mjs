// Erzeugt die Lizenzangaben der egui-Vorschau von Daedyx Tasks (aus Desk übernommen, Tasks T6) aus `cargo metadata` (ohne Netz, `--offline`):
// - `crates/tasks-app/lizenzen.tsv`: alle Bibliotheken, die das Programm unter Windows wirklich mitbaut (normale
//   Abhängigkeiten ab `daedyx-tasks-app`, ohne eigene Pfad-Pakete), dazu fest mitgebauter C-Code (SQLCipher, OpenSSL).
//   Spalten: Name, Version, Lizenz.
// - `crates/tasks-app/lizenztexte.txt`: je Eintrag Urheber und die Lizenzdateien aus dem Paket (LICENSE, COPYING,
//   NOTICE …), gleiche Texte nur einmal im Anhang. Das erfüllt die Hinweispflichten (Review int2 M-1).
// - `crates/tasks-app/lizenzen-ausgenommen.tsv`: Pakete aus Cargo.lock, die nicht ins Programm kommen (anderes
//   System, nur Bau oder Test). Ein Test prüft, dass jedes Paket aus Cargo.lock in genau einer der Listen steht.
// Aufruf aus dem Repo-Ordner: node scripts/vorschau-lizenzen.mjs
import { execFileSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const root = join(import.meta.dirname, "..", "app", "src-tauri");
const out = join(root, "crates", "tasks-app");
const meta = JSON.parse(
  execFileSync(
    "cargo",
    ["metadata", "--offline", "--locked", "--format-version", "1", "--filter-platform", "x86_64-pc-windows-msvc"],
    { cwd: root, maxBuffer: 256 * 1024 * 1024, encoding: "utf8" },
  ),
);
const packages = new Map(meta.packages.map((p) => [p.id, p]));
const nodes = new Map(meta.resolve.nodes.map((n) => [n.id, n]));
const start = meta.packages.find((p) => p.name === "daedyx-tasks-app");
if (!start) throw new Error("daedyx-tasks-app nicht gefunden");

// Build-Abhängigkeiten, deren C-Code fest ins Programm gelinkt wird.
const VENDORED_BUILD = new Set(["openssl-src"]);
const seen = new Set([start.id]);
const queue = [start.id];
while (queue.length > 0) {
  const node = nodes.get(queue.shift());
  for (const dep of node?.deps ?? []) {
    const normal = dep.dep_kinds.some((k) => k.kind === null);
    const vendored = packages.get(dep.pkg) && VENDORED_BUILD.has(packages.get(dep.pkg).name);
    if ((normal || vendored) && !seen.has(dep.pkg)) {
      seen.add(dep.pkg);
      queue.push(dep.pkg);
    }
  }
}

const shipped = [...seen].map((id) => packages.get(id)).filter((p) => p && p.source !== null);
const byName = (n) => shipped.find((p) => p.name === n);
const licenseFile = /^(licen[cs]e|copying|notice|copyright|unlicense|ofl|ufl)/i;

function filesOf(dir, depth = 0) {
  const found = [];
  for (const f of readdirSync(dir).sort()) {
    const path = join(dir, f);
    const st = statSync(path);
    if (st.isFile() && licenseFile.test(f)) found.push(path);
    if (st.isDirectory() && depth === 0 && f === "fonts") {
      for (const g of readdirSync(path).sort()) {
        if (g.endsWith(".txt")) found.push(join(path, g));
      }
    }
  }
  return found;
}

// Pakete ohne eigene Lizenzdatei: Standardtext jeder genannten Lizenz aus `scripts/lizenztexte/<SPDX>.txt`;
// den Urhebervermerk liefert dann die Zeile „Urheber“ (Angaben des Pakets).
const standard = join(import.meta.dirname, "lizenztexte");
function standardTexts(license) {
  return [...new Set(license.split(/[\s()]+/).filter((id) => id && id !== "OR" && id !== "AND"))]
    .map((id) => join(standard, `${id}.txt`))
    .filter((f) => existsSync(f));
}

const entries = shipped.map((p) => {
  const license = (p.license ?? "").replace(/\//g, " OR ") || "?";
  const own = filesOf(dirname(p.manifest_path));
  return {
    name: p.name,
    version: p.version,
    license,
    // Für den Urhebervermerk genügt der Name; Mailadressen aus den Paketdaten bleiben draußen.
    // Eigene Bausteine aus daedyx/ui und daedyx/core stehen nicht auf crates.io: Urheber ist Daedyx.
    authors: p.name.startsWith("daedyx-")
      ? ["Daedyx"]
      : (p.authors ?? []).map((a) => a.replace(/\s*<[^>]*>/g, "").replace(/\S+@\S+/g, "").trim()).filter(Boolean),
    files: own.length > 0 ? own : standardTexts(license),
  };
});

// Fest mitgebauter C-Code: eigene Einträge mit seinen Lizenztexten (die Crate-Lizenz deckt ihn nicht ab).
const sqlite = byName("libsqlite3-sys");
if (sqlite) {
  const lic = join(dirname(sqlite.manifest_path), "sqlcipher", "LICENSE");
  if (!existsSync(lic)) throw new Error(`SQLCipher-Lizenz fehlt: ${lic}`);
  entries.push({
    name: "SQLCipher (C-Code in libsqlite3-sys)",
    version: sqlite.version,
    license: "BSD-artig (Zetetic LLC); SQLite: gemeinfrei",
    authors: ["Zetetic LLC"],
    files: [lic],
  });
}
const ossl = byName("openssl-src");
if (ossl) {
  const lic = join(dirname(ossl.manifest_path), "openssl", "LICENSE.txt");
  if (!existsSync(lic)) throw new Error(`OpenSSL-Lizenz fehlt: ${lic}`);
  entries.push({
    name: "OpenSSL (C-Code in openssl-src)",
    version: ossl.version.split("+").pop(),
    license: "Apache-2.0",
    authors: ["The OpenSSL Project Authors"],
    files: [lic],
  });
}
entries.sort((a, b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version, undefined, { numeric: true }));

const missing = entries.filter((e) => e.files.length === 0);
if (missing.length > 0) {
  throw new Error(`Ohne Lizenzdatei: ${missing.map((e) => `${e.name} ${e.version}`).join(", ")}`);
}

// Gleiche Texte nur einmal (Apache-2.0 steht sonst hundertfach darin).
const texts = new Map();
const lines = [
  "Lizenzhinweise der Bibliotheken in Daedyx Tasks (Vorschau)",
  "",
  "Daedyx Tasks selbst steht unter der MIT-Lizenz. Das Programm enthält Bibliotheken anderer Urheber unter deren",
  "Lizenzen. Für jede Bibliothek folgen Name, Version, Lizenz, Urheber und Verweise auf die Lizenztexte im Anhang.",
  "",
  "Bestandteile unter MPL-2.0 sind unverändert übernommen. Ihr Quelltext ist unter https://crates.io/crates/<Name>/<Version>",
  "erhältlich (genaue Version siehe Eintrag). Die Stammzertifikate von webpki-roots stehen unter der",
  "Community Data License Agreement – Permissive 2.0 (Text im Anhang). Die eingebauten Schriften (epaint_default_fonts)",
  "stehen unter der SIL Open Font License 1.1 und der Ubuntu Font Licence 1.0 und werden nicht einzeln verkauft.",
  "",
];
for (const e of entries) {
  const refs = e.files.map((f) => {
    const body = readFileSync(f, "utf8").replace(/\r\n/g, "\n").trim();
    if (!texts.has(body)) texts.set(body, `T${texts.size + 1}`);
    return `[${texts.get(body)}]`;
  });
  lines.push(`== ${e.name} ${e.version} ==`);
  lines.push(`lizenz: ${e.license}`);
  lines.push(`urheber: ${e.authors.length > 0 ? e.authors.join(", ") : `keine Angabe im Paket, siehe https://crates.io/crates/${e.name}/${e.version}`}`);
  lines.push(`texte: ${refs.join(" ")}`);
  lines.push("");
}
lines.push("=== Lizenztexte ===", "");
for (const [body, id] of texts) {
  lines.push(`--- [${id}] ---`, body, "");
}

writeFileSync(join(out, "lizenzen.tsv"), entries.map((e) => [e.name, e.version, e.license].join("\t")).join("\n") + "\n", "utf8");
writeFileSync(join(out, "lizenztexte.txt"), lines.join("\n"), "utf8");

// Alles aus Cargo.lock, was nicht mitgebaut wird.
const lock = readFileSync(join(root, "Cargo.lock"), "utf8").replace(/\r\n/g, "\n");
const inLock = [...lock.matchAll(/\[\[package\]\]\nname = "([^"]+)"\nversion = "([^"]+)"\nsource = /g)].map((m) => [m[1], m[2]]);
const shippedKey = new Set(shipped.map((p) => `${p.name} ${p.version}`));
const excluded = inLock.filter(([n, v]) => !shippedKey.has(`${n} ${v}`)).sort((a, b) => a[0].localeCompare(b[0]));
writeFileSync(join(out, "lizenzen-ausgenommen.tsv"), excluded.map((r) => r.join("\t")).join("\n") + "\n", "utf8");
console.log(`${entries.length} Einträge, ${texts.size} Lizenztexte, ${excluded.length} ausgenommen`);
