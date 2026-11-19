// Einmalig beim Ableiten aus Desk: behält in de.json nur die Schlüssel, die der Quellcode von Tasks nennt.
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..", "app", "src");
const files = [];
(function walk(d) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    if (statSync(p).isDirectory()) walk(p);
    else if (/\.(ts|tsx)$/.test(n)) files.push(p);
  }
})(root);
const text = files.map((f) => readFileSync(f, "utf8")).join("\n");
const dictPath = join(root, "i18n", "de.json");
const dict = JSON.parse(readFileSync(dictPath, "utf8"));
const prefixes = [...text.matchAll(/`([a-z_]+(?:\.[a-z_0-9]+)*\.?)\$\{/g)].map((m) => m[1]);
const keep = Object.fromEntries(
  Object.entries(dict).filter(([k]) => text.includes(`"${k}"`) || prefixes.some((p) => k.startsWith(p))),
);
writeFileSync(dictPath, JSON.stringify(keep, null, 2) + "\n");
console.log(`${Object.keys(dict).length} -> ${Object.keys(keep).length}`);
