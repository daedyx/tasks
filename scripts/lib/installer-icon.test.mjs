import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const tauri = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "app", "src-tauri");
const conf = JSON.parse(readFileSync(join(tauri, "tauri.conf.json"), "utf8"));

describe("Installer-Symbol", () => {
  it("der NSIS-Installer trägt das eigene Symbol statt des Standard-Symbols", () => {
    const icon = conf.bundle.windows.nsis.installerIcon;
    expect(icon).toBe("icons/icon.ico");
    // gleiche Datei wie das App-Symbol, und es ist wirklich eine .ico-Datei (Kopf 00 00 01 00)
    expect(conf.bundle.icon).toContain(icon);
    const head = readFileSync(join(tauri, icon)).subarray(0, 4);
    expect([...head]).toEqual([0, 0, 1, 0]);
  });
});
