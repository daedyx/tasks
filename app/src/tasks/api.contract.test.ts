import { existsSync, readdirSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { TASK_ERROR_CODES } from "./errors";
import de from "../i18n/de.json";

const root = join(__dirname, "..", "..");
const read = (...p: string[]) => readFileSync(join(root, ...p), "utf8");

const api = read("src", "tasks", "api.ts");
const lib = read("src-tauri", "src", "lib.rs");
// Die Fehlercodes liegen im Kern (Git-Abhängigkeit). Quelle: Nachbarordner `daedyx-core` oder die Cargo-Kopie.
const coreErrors = (): string | null => {
  const rel = ["crates", "daedyx-tasks-core", "src", "errors.rs"];
  const dirs = [join(root, "..", "..", "daedyx-core")];
  const git = join(homedir(), ".cargo", "git", "checkouts");
  if (existsSync(git)) {
    for (const repo of readdirSync(git).filter((d) => d.startsWith("core-"))) {
      for (const rev of readdirSync(join(git, repo))) dirs.push(join(git, repo, rev));
    }
  }
  const hit = dirs.map((d) => join(d, ...rel)).find((f) => existsSync(f));
  return hit ? readFileSync(hit, "utf8") : null;
};
const errors = coreErrors();

const called = [...api.matchAll(/call(?:<[^>]*>)?\(\s*"(task_[a-z_]+)"/g)].map((m) => m[1]);
const registered = [...lib.matchAll(/commands_tasks::(task_[a-z_]+)/g)].map((m) => m[1]);

describe("Schnittstelle Oberfläche ↔ Kern (Aufgaben)", () => {
  it("ruft nur Befehle auf, die der Kern registriert hat, und umgekehrt", () => {
    expect(called.length).toBeGreaterThan(10);
    expect([...new Set(called)].sort()).toEqual([...new Set(registered)].sort());
  });

  it.skipIf(!errors)("kennt jeden Fehlercode des Kerns und hat dafür einen deutschen Text", () => {
    const codes = [...(errors ?? "").matchAll(/TaskError::\w+(?:\(_\))? => "([a-z_]+)"/g)].map((m) => m[1]);
    expect(codes.length).toBe(6);
    for (const code of codes) {
      expect(TASK_ERROR_CODES).toContain(code);
      expect(de).toHaveProperty([`tasks.error.${code}`]);
    }
    expect(de).toHaveProperty(["tasks.error.internal"]);
  });

  it("jeder Befehlsname der Befehlsdatei steht in der Befehlsliste", () => {
    const file = read("src-tauri", "src", "commands_tasks.rs");
    const defined = [...file.matchAll(/pub fn (task_[a-z_]+)\(/g)].map((m) => m[1]);
    expect(defined.sort()).toEqual([...new Set(registered)].sort());
  });
});
