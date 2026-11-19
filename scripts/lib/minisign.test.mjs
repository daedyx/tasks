import { createHash, createPublicKey, verify } from "node:crypto";
import { describe, expect, it } from "vitest";
import { generateKeyPair, signText } from "./minisign.mjs";

const b64 = (line) => Buffer.from(line, "base64");

describe("minisign", () => {
  it("erzeugt Schlüsseldateien im minisign-Format", () => {
    const { publicKeyText, secretKeyText } = generateKeyPair();
    const pub = publicKeyText.split("\n");
    expect(pub[0]).toMatch(/^untrusted comment: minisign public key: [0-9A-F]{16}$/);
    const raw = b64(pub[1]);
    expect(raw.length).toBe(42);
    expect(raw.subarray(0, 2).toString()).toBe("Ed");
    const sec = secretKeyText.split("\n");
    expect(sec[0]).toBe("untrusted comment: minisign encrypted secret key");
    expect(b64(sec[1]).length).toBe(158);
  });

  it("signiert so, dass die Signatur mit dem öffentlichen Schlüssel prüfbar ist (ED, Vorab-Hash)", () => {
    const { publicKeyText, secretKeyText } = generateKeyPair();
    const content = '{"schema":1}';
    const sig = signText(content, secretKeyText, "trusted").split("\n");
    expect(sig[0]).toMatch(/^untrusted comment: /);
    expect(sig[2]).toBe("trusted comment: trusted");

    const pubRaw = b64(publicKeyText.split("\n")[1]);
    const sigRaw = b64(sig[1]);
    expect(sigRaw.subarray(0, 2).toString()).toBe("ED");
    expect(sigRaw.subarray(2, 10).equals(pubRaw.subarray(2, 10))).toBe(true); // gleiche Schlüsselkennung

    const key = createPublicKey({
      key: Buffer.concat([Buffer.from("302a300506032b6570032100", "hex"), pubRaw.subarray(10)]),
      format: "der",
      type: "spki",
    });
    const hash = createHash("blake2b512").update(content).digest();
    expect(verify(null, hash, key, sigRaw.subarray(10))).toBe(true);
    // globale Signatur über Signatur + vertrauenswürdigen Kommentar
    const global = verify(null, Buffer.concat([sigRaw.subarray(10), Buffer.from("trusted")]), key, b64(sig[3]));
    expect(global).toBe(true);
  });

  it("eine veränderte Datei besteht die Prüfung nicht", () => {
    const { publicKeyText, secretKeyText } = generateKeyPair();
    const sigRaw = b64(signText("a", secretKeyText, "t").split("\n")[1]);
    const pubRaw = b64(publicKeyText.split("\n")[1]);
    const key = createPublicKey({
      key: Buffer.concat([Buffer.from("302a300506032b6570032100", "hex"), pubRaw.subarray(10)]),
      format: "der",
      type: "spki",
    });
    expect(verify(null, createHash("blake2b512").update("b").digest(), key, sigRaw.subarray(10))).toBe(false);
  });

  it("lehnt eine kaputte Schlüsseldatei ab", () => {
    expect(() => signText("a", "Unsinn", "t")).toThrow(/Schlüssel/);
  });
});
