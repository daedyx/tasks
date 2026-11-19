import { createHash, createPublicKey, verify } from "node:crypto";
import { mkdtempSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { updateLocalCatalog } from "./catalog-file.mjs";
import { generateKeyPair } from "./minisign.mjs";

const APP = { id: "desk", name: "Daedyx Desk", description: "Mail, Kalender, Aufgaben", license: "", author: "Daedyx" };

function setup() {
  const root = mkdtempSync(join(tmpdir(), "daedyx-cat-"));
  const setupFile = join(root, "Daedyx.Desk_0.4.0_x64-setup.exe");
  writeFileSync(setupFile, "abc");
  const keys = generateKeyPair();
  return { root, catalogDir: join(root, "catalog"), setupFile, keys };
}

const call = (s, version, extra = {}) =>
  updateLocalCatalog({
    catalogDir: s.catalogDir,
    secretKeyText: s.keys.secretKeyText,
    app: APP,
    version,
    date: "2026-11-20",
    notes: "n",
    setupPath: s.setupFile,
    uploadName: "Daedyx.Desk_0.4.0_x64-setup.exe",
    ...extra,
  });

describe("lokaler Katalog", () => {
  it("legt Katalog, Paket und Signatur an", () => {
    const s = setup();
    call(s, "0.4.0");
    const json = readFileSync(join(s.catalogDir, "catalog.json"), "utf8");
    const cat = JSON.parse(json);
    expect(cat.schema).toBe(1);
    expect(cat.minLauncherVersion).toBe("0.1.0");
    expect(cat.apps).toHaveLength(1);
    expect(cat.apps[0]).toMatchObject({ id: "desk", name: "Daedyx Desk", description: "Mail, Kalender, Aufgaben" });
    const pkg = cat.apps[0].releases[0].packages["windows-x64"];
    expect(pkg).toEqual({
      url: "packages/Daedyx.Desk_0.4.0_x64-setup.exe",
      sha256: createHash("sha256").update("abc").digest("hex"),
      size: 3,
      kind: "nsis-silent",
    });
    expect(readFileSync(join(s.catalogDir, "packages", "Daedyx.Desk_0.4.0_x64-setup.exe"), "utf8")).toBe("abc");

    // Signatur ist über genau diese Datei gültig
    const sig = readFileSync(join(s.catalogDir, "catalog.json.minisig"), "utf8").split("\n");
    const sigRaw = Buffer.from(sig[1], "base64");
    const pubRaw = Buffer.from(s.keys.publicKeyText.split("\n")[1], "base64");
    const key = createPublicKey({
      key: Buffer.concat([Buffer.from("302a300506032b6570032100", "hex"), pubRaw.subarray(10)]),
      format: "der",
      type: "spki",
    });
    expect(verify(null, createHash("blake2b512").update(json).digest(), key, sigRaw.subarray(10))).toBe(true);
  });

  it("eine zweite Version kommt dazu, dieselbe Version wird ersetzt, andere Apps bleiben", () => {
    const s = setup();
    call(s, "0.4.0");
    // fremde App im Katalog (zum Beispiel vom Launcher-Entwickler)
    const path = join(s.catalogDir, "catalog.json");
    const cat = JSON.parse(readFileSync(path, "utf8"));
    cat.apps.push({ id: "board", name: "Board", releases: [] });
    writeFileSync(path, JSON.stringify(cat));
    call(s, "0.4.1", { notes: "neu" });
    call(s, "0.4.1", { notes: "überarbeitet" });
    const out = JSON.parse(readFileSync(path, "utf8"));
    expect(out.apps.map((a) => a.id)).toEqual(["desk", "board"]);
    const versions = out.apps[0].releases.map((r) => r.version);
    expect(versions).toEqual(["0.4.0", "0.4.1"]);
    expect(out.apps[0].releases[1].notes).toBe("überarbeitet");
  });

  it("ein beschädigter Katalog wird nicht überschrieben", () => {
    const s = setup();
    call(s, "0.4.0");
    writeFileSync(join(s.catalogDir, "catalog.json"), "{kaputt");
    expect(() => call(s, "0.4.1")).toThrow(/Katalog/);
    expect(readFileSync(join(s.catalogDir, "catalog.json"), "utf8")).toBe("{kaputt");
  });

  it("ohne gültigen Schlüssel wird nichts geschrieben", () => {
    const s = setup();
    expect(() => call(s, "0.4.0", { secretKeyText: "Unsinn" })).toThrow(/Schlüssel/);
    expect(existsSync(join(s.catalogDir, "catalog.json"))).toBe(false);
  });
});
