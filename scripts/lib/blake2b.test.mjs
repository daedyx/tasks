import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import { blake2b } from "./blake2b.mjs";

describe("blake2b", () => {
  it("BLAKE2b-256 von „abc“ (Testvektor)", () => {
    expect(blake2b("abc", 32).toString("hex")).toBe("bddd813c634239723171ef3fee98579b94964e3bb1cb3e427262c8c068d52319");
  });

  it("stimmt bei 64 Byte Ausgabe mit Nodes eigener Funktion überein, auch über Blockgrenzen", () => {
    for (const n of [0, 1, 127, 128, 129, 300]) {
      const data = Buffer.alloc(n, 7);
      expect(blake2b(data, 64).toString("hex")).toBe(createHash("blake2b512").update(data).digest("hex"));
    }
  });
});
