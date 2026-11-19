// Minimale, minisign-kompatible Schlüsselerzeugung und Signatur (Algorithmus "ED": Ed25519 über BLAKE2b-512).
// Die Secret-Key-Datei wird unverschlüsselt geschrieben (kein Passwort nötig, Schutz durch Dateirechte/Ablageort).
import { createHash, createPrivateKey, generateKeyPairSync, randomBytes, sign } from "node:crypto";
import { blake2b as blake2bAny } from "./blake2b.mjs";

const PKCS8_PREFIX = Buffer.from("302e020100300506032b657004220420", "hex");
const SPKI_PREFIX = Buffer.from("302a300506032b6570032100", "hex");

function blake2b(bytes, bits) {
  return bits === 512 ? createHash("blake2b512").update(bytes).digest() : blake2bAny(bytes, bits / 8);
}

function privateKeyFromSeed(seed) {
  return createPrivateKey({ key: Buffer.concat([PKCS8_PREFIX, seed]), format: "der", type: "pkcs8" });
}

export function generateKeyPair() {
  const { publicKey, privateKey } = generateKeyPairSync("ed25519");
  const pk = publicKey.export({ format: "der", type: "spki" }).subarray(SPKI_PREFIX.length);
  const seed = privateKey.export({ format: "der", type: "pkcs8" }).subarray(PKCS8_PREFIX.length);
  const keyId = randomBytes(8);
  const sk = Buffer.concat([seed, pk]);
  const checksum = blake2b(Buffer.concat([Buffer.from("Ed"), keyId, sk]), 256);
  const idHex = Buffer.from(keyId).reverse().toString("hex").toUpperCase();
  const secretBox = Buffer.concat([
    Buffer.from("Ed"),
    Buffer.alloc(2), // keine Schlüsselableitung: unverschlüsselt
    Buffer.from("B2"),
    Buffer.alloc(32 + 8 + 8),
    keyId,
    sk,
    checksum,
  ]);
  return {
    publicKeyText: `untrusted comment: minisign public key: ${idHex}\n${Buffer.concat([Buffer.from("Ed"), keyId, pk]).toString("base64")}\n`,
    secretKeyText: `untrusted comment: minisign encrypted secret key\n${secretBox.toString("base64")}\n`,
  };
}

function readSecret(text) {
  const lines = text.split(/\r?\n/);
  const raw = Buffer.from(lines[1] ?? "", "base64");
  if (!lines[0]?.startsWith("untrusted comment:") || raw.length !== 158 || raw.subarray(0, 2).toString() !== "Ed") {
    throw new Error("Ungültige Schlüsseldatei (minisign-Format erwartet)");
  }
  if (!raw.subarray(2, 4).equals(Buffer.alloc(2))) {
    throw new Error("Schlüssel ist passwortgeschützt; dieses Werkzeug kennt nur unverschlüsselte Schlüssel");
  }
  const body = raw.subarray(54);
  const keyId = body.subarray(0, 8);
  const sk = body.subarray(8, 72);
  const checksum = blake2b(Buffer.concat([Buffer.from("Ed"), keyId, sk]), 256);
  if (!checksum.equals(body.subarray(72))) throw new Error("Ungültige Schlüsseldatei (Prüfsumme)");
  return { keyId, key: privateKeyFromSeed(sk.subarray(0, 32)) };
}

/** Signatur im minisign-Dateiformat (`.minisig`) über den Text `content`. */
export function signText(content, secretKeyText, trustedComment) {
  const { keyId, key } = readSecret(secretKeyText);
  const sig = sign(null, blake2b(Buffer.from(content), 512), key);
  const global = sign(null, Buffer.concat([sig, Buffer.from(trustedComment)]), key);
  const body = Buffer.concat([Buffer.from("ED"), keyId, sig]).toString("base64");
  return `untrusted comment: signature from minisign secret key\n${body}\ntrusted comment: ${trustedComment}\n${global.toString("base64")}\n`;
}
