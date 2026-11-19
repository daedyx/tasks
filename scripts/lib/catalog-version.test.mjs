import { describe, expect, it } from "vitest";
import { assertNewer } from "./catalog-version.mjs";

const catalog = (versions) => JSON.stringify({ schema: 1, minLauncherVersion: "0.1.0", apps: [{ id: "desk", releases: versions.map((v) => ({ version: v })) }] });

describe("assertNewer", () => {
  it("erlaubt die erste Version in einem leeren oder fehlenden Katalog", () => {
    expect(() => assertNewer(null, "desk", "0.5.0")).not.toThrow();
    expect(() => assertNewer(JSON.stringify({ schema: 1, apps: [] }), "desk", "0.5.0")).not.toThrow();
  });

  it("erlaubt eine höhere Version, auch wenn Zahlen mehrstellig sind (0.10 > 0.9)", () => {
    expect(() => assertNewer(catalog(["0.4.0", "0.9.0"]), "desk", "0.10.0")).not.toThrow();
    expect(() => assertNewer(catalog(["0.5.0"]), "desk", "0.5.1")).not.toThrow();
  });

  it("lehnt dieselbe und niedrigere Versionen ab", () => {
    expect(() => assertNewer(catalog(["0.5.0"]), "desk", "0.5.0")).toThrow(/höher/);
    expect(() => assertNewer(catalog(["0.5.0", "0.6.0"]), "desk", "0.5.5")).toThrow(/höher/);
  });

  it("andere Apps zählen nicht", () => {
    const c = JSON.stringify({ schema: 1, apps: [{ id: "board", releases: [{ version: "9.0.0" }] }] });
    expect(() => assertNewer(c, "desk", "0.5.0")).not.toThrow();
  });

  it("lehnt eine ungültige Version und einen kaputten Katalog ab", () => {
    expect(() => assertNewer(null, "desk", "v5")).toThrow(/Version/);
    expect(() => assertNewer("{kaputt", "desk", "0.5.0")).toThrow(/Katalog/);
  });
});
