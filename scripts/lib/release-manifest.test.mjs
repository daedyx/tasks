import { describe, expect, it } from "vitest";
import { buildCatalogRelease, buildLatestJson, sha256Hex } from "./release-manifest.mjs";

const base = { version: "0.4.0", date: "2026-11-20", notes: "Erste Version", url: "https://example.org/dl/desk-0.4.0.exe", size: 5 };
const sha = "aa".repeat(32);

describe("release-manifest", () => {
  it("sha256 ist kleingeschrieben hexadezimal", () => {
    expect(sha256Hex(Buffer.from("abc"))).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
  });

  it("Katalog-Eintrag passt zum Launcher-Schema 1", () => {
    expect(buildCatalogRelease({ ...base, sha256: sha })).toEqual({
      version: "0.4.0",
      date: "2026-11-20",
      notes: "Erste Version",
      packages: { "windows-x64": { url: base.url, sha256: sha, size: 5, kind: "nsis-silent" } },
    });
  });

  it("latest.json hat das Format des Tauri-Updaters und dieselbe Adresse und Version", () => {
    const l = buildLatestJson({ ...base, signature: "SIGNATUR" });
    expect(l.version).toBe("0.4.0");
    expect(l.platforms["windows-x86_64"]).toEqual({ signature: "SIGNATUR", url: base.url });
    expect(l.pub_date).toBe("2026-11-20T00:00:00Z");
  });

  it("lehnt ungültige Version, Prüfsumme und Adressen ohne HTTPS ab", () => {
    expect(() => buildCatalogRelease({ ...base, version: "v4", sha256: sha })).toThrow(/Version/);
    expect(() => buildCatalogRelease({ ...base, sha256: "XY" })).toThrow(/sha256/);
    expect(() => buildCatalogRelease({ ...base, url: "http://example.org/x.exe", sha256: sha })).toThrow(/https/);
  });

  it("erlaubt http nur für 127.0.0.1 und nur mit ausdrücklicher Option", () => {
    const local = { ...base, url: "http://127.0.0.1:8765/x.exe", sha256: sha };
    expect(() => buildCatalogRelease(local)).toThrow(/https/);
    expect(() => buildCatalogRelease(local, { allowLocalHttp: true })).not.toThrow();
    const evil = { ...local, url: "http://127.0.0.1.evil.example/x.exe" };
    expect(() => buildCatalogRelease(evil, { allowLocalHttp: true })).toThrow(/https/);
  });
});

describe("relative Adressen (lokaler Katalog)", () => {
  const rel = { ...base, url: "packages/Daedyx.Desk_0.4.0_x64-setup.exe", sha256: sha };

  it("sind nur mit ausdrücklicher Option erlaubt", () => {
    expect(() => buildCatalogRelease(rel)).toThrow(/https/);
    expect(() => buildCatalogRelease(rel, { allowRelative: true })).not.toThrow();
  });

  it("dürfen den Katalogordner nicht verlassen", () => {
    for (const bad of ["../x.exe", "/x.exe", "C:\\x.exe", "packages/../../x.exe", "packages\\x.exe"]) {
      expect(() => buildCatalogRelease({ ...rel, url: bad }, { allowRelative: true }), bad).toThrow(/Adresse/);
    }
  });
});
