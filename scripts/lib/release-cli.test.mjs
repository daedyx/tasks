import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { run } from "./release-cli.mjs";
import { generateKeyPair } from "./minisign.mjs";

function setup(withSig = true) {
  const root = mkdtempSync(join(tmpdir(), "daedyx-rel-"));
  const dir = join(root, "bundle");
  mkdirSync(dir);
  const file = join(dir, "Daedyx Tasks_0.4.0_x64-setup.exe");
  writeFileSync(file, "abc");
  if (withSig) writeFileSync(`${file}.sig`, "SIGNATUR\n");
  return { dir, out: join(root, "out") };
}

const opts = { version: "0.4.0", notes: "n", baseUrl: "https://example.org/dl/", date: "2026-11-20" };

describe("release-cli", () => {
  it("schreibt Setup, latest.json und Katalog-Eintrag mit gleicher Adresse und Prüfsumme", () => {
    const { dir, out } = setup();
    run({ ...opts, dir, out });
    const url = "https://example.org/dl/Daedyx.Tasks_0.4.0_x64-setup.exe";
    const latest = JSON.parse(readFileSync(join(out, "latest.json"), "utf8"));
    expect(latest.platforms["windows-x86_64"]).toEqual({ signature: "SIGNATUR", url });
    const entry = JSON.parse(readFileSync(join(out, "catalog-entry.json"), "utf8"));
    expect(entry.packages["windows-x64"]).toEqual({
      url,
      sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
      size: 3,
      kind: "nsis-silent",
    });
    expect(existsSync(join(out, "Daedyx.Tasks_0.4.0_x64-setup.exe"))).toBe(true);
  });

  it("bricht ohne Signatur ab", () => {
    const { dir, out } = setup(false);
    expect(() => run({ ...opts, dir, out })).toThrow(/Signatur fehlt/);
  });

  it("bricht ab, wenn das Setup zur Version fehlt", () => {
    const { dir, out } = setup();
    expect(() => run({ ...opts, version: "0.5.0", dir, out })).toThrow(/Setup/);
  });

  it("schreibt nur in den lokalen Katalog, wenn keine Adresse und keine Update-Signatur da sind", () => {
    const { dir } = setup(false);
    const root = mkdtempSync(join(tmpdir(), "daedyx-keys-"));
    const catalogKey = join(root, "catalog.key");
    writeFileSync(catalogKey, generateKeyPair().secretKeyText);
    const catalogDir = join(root, "catalog");
    run({ version: "0.4.0", notes: "n", date: "2026-11-20", dir, catalogDir, catalogKey });
    const cat = JSON.parse(readFileSync(join(catalogDir, "catalog.json"), "utf8"));
    expect(cat.apps[0].id).toBe("tasks");
    expect(cat.apps[0].registryKey).toBe("Daedyx Tasks"); // Uninstall-Eintrag der Registry, damit der Launcher eine vorhandene Installation findet
    expect(cat.apps[0].releases[0].packages["windows-x64"].url).toBe("packages/Daedyx.Tasks_0.4.0_x64-setup.exe");
    expect(existsSync(join(catalogDir, "catalog.json.minisig"))).toBe(true);
    expect(existsSync(join(catalogDir, "latest.json"))).toBe(false);
  });

  it("ohne Ziel (weder Adresse noch Katalog) ist das ein Fehler", () => {
    const { dir } = setup(false);
    expect(() => run({ version: "0.4.0", date: "2026-11-20", dir })).toThrow(/Ziel/);
  });
});
