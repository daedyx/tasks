// Erzeugt aus einer Veröffentlichung zwei Ausgaben aus einer Quelle:
// den Katalog-Eintrag (Daedyx Launcher, Schema 1) und die latest.json (Tauri-Updater).
import { createHash } from "node:crypto";

const VERSION = /^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/;
const LOCAL_HTTP = /^http:\/\/127\.0\.0\.1(:\d+)?\//;

export function sha256Hex(buffer) {
  return createHash("sha256").update(buffer).digest("hex");
}

function check({ version, sha256, url }, { allowLocalHttp = false, allowRelative = false } = {}) {
  if (!VERSION.test(version)) throw new Error(`Ungültige Version: ${version}`);
  if (sha256 !== undefined && !/^[0-9a-f]{64}$/.test(sha256)) {
    throw new Error("sha256 muss 64 Kleinbuchstaben oder Ziffern haben");
  }
  if (allowRelative && !/^[a-z]+:/i.test(url)) {
    const safe = /^[\w.-]+(\/[\w.-]+)*$/.test(url) && !url.split("/").includes("..");
    if (!safe) throw new Error(`Ungültige relative Adresse: ${url}`);
    return;
  }
  const local = allowLocalHttp && LOCAL_HTTP.test(url);
  if (!url.startsWith("https://") && !local) throw new Error("Adresse muss mit https:// beginnen");
}

export function buildCatalogRelease(a, opts) {
  check(a, opts);
  return {
    version: a.version,
    date: a.date,
    notes: a.notes ?? "",
    packages: { "windows-x64": { url: a.url, sha256: a.sha256, size: a.size, kind: "nsis-silent" } },
  };
}

export function buildLatestJson(a, opts) {
  check({ version: a.version, url: a.url }, opts);
  return {
    version: a.version,
    notes: a.notes ?? "",
    pub_date: `${a.date}T00:00:00Z`,
    platforms: { "windows-x86_64": { signature: a.signature, url: a.url } },
  };
}
