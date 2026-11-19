// Kommandozeile von release-cli.mjs. Notizen kommen aus einer Datei (--notes-file): Werte mit Anführungszeichen,
// Zeilenumbrüchen oder führendem "--" überstehen die Übergabe von PowerShell an node sonst nicht.
import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";

export function parseCli(argv) {
  const { values } = parseArgs({
    args: argv,
    strict: true,
    options: {
      dir: { type: "string" },
      out: { type: "string" },
      version: { type: "string" },
      "notes-file": { type: "string" },
      "base-url": { type: "string" },
      date: { type: "string" },
      "allow-local-http": { type: "string" },
      "catalog-dir": { type: "string" },
      "catalog-key": { type: "string" },
    },
  });
  return {
    dir: values.dir,
    out: values.out,
    version: values.version,
    notes: values["notes-file"] ? readFileSync(values["notes-file"], "utf8").replace(/^﻿/, "").trim() : "",
    baseUrl: values["base-url"],
    date: values.date ?? new Date().toISOString().slice(0, 10),
    allowLocalHttp: values["allow-local-http"] === "true",
    catalogDir: values["catalog-dir"],
    catalogKey: values["catalog-key"],
  };
}
