import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import { describe, expect, it } from "vitest";
import de from "./de.json";
import { t } from "./index";

function sourceFiles(dir: string, ext = /\.(ts|tsx)$/): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const path = join(dir, e.name);
    if (e.isDirectory()) return sourceFiles(path, ext);
    return ext.test(e.name) && !/\.test\./.test(e.name) ? [path] : [];
  });
}

const SRC = join(__dirname, "..");
const keys = Object.keys(de);
const namespaces = [...new Set(keys.map((k) => k.split(".")[0]))];
/** Zeichenketten in "…", '…' oder `…`, die wie ein Schlüssel beginnen (Namensraum + Punkt) */
const KEY_LITERAL = new RegExp(`(["'\`])((?:${namespaces.join("|")})[.][A-Za-z0-9_.$\\{\\}]+)\\1`, "g");

describe("t", () => {
  it("liefert den Text zum Schlüssel", () => {
    expect(t("settings.title")).toBe("Einstellungen");
  });

  it("ersetzt Platzhalter", () => {
    expect(t("updates.version", { version: "1.2.3" })).toBe("Installierte Version: 1.2.3");
  });

  it("gibt bei unbekanntem Schlüssel den Schlüssel zurück", () => {
    expect(t("gibt.es.nicht")).toBe("gibt.es.nicht");
  });
});

describe("Sprachdatei", () => {
  it("enthält jeden im Code verwendeten Schlüssel (auch in Backticks und Schablonen)", () => {
    const missing: string[] = [];
    for (const file of sourceFiles(SRC)) {
      for (const [, , key] of readFileSync(file, "utf8").matchAll(KEY_LITERAL)) {
        const at = key.indexOf("${");
        if (at === -1) {
          if (!(key in de)) missing.push(`${file}: ${key}`);
        } else if (!keys.some((k) => k.startsWith(key.slice(0, at)))) {
          // Schablone wie `mail.error.${code}`: mindestens ein Schlüssel muss mit dem festen Teil beginnen
          missing.push(`${file}: ${key} (kein Schlüssel mit diesem Anfang)`);
        }
      }
    }
    expect(missing).toEqual([]);
  });

  it("erkennt Schlüssel in Backticks und Schablonen (Selbsttest der Wache)", () => {
    const text = "t(`mail.folder.${role}`) t(`mail.gibt.es.nicht`) t('mail.error.auth')";
    expect([...text.matchAll(KEY_LITERAL)].map((m) => m[2])).toEqual([
      "mail.folder.${role}",
      "mail.gibt.es.nicht",
      "mail.error.auth",
    ]);
  });
});

/** Attribute, deren Text ein Mensch liest */
const TEXT_ATTRIBUTES = new Set(["aria-label", "title", "placeholder", "alt"]);
/** Sichtbare Texte, die bewusst nicht übersetzt werden (Markenname) */
const ALLOWED_TEXT = new Set(["Daedyx Tasks"]);

function literalTexts(file: string, code: string = readFileSync(file, "utf8")): string[] {
  const source = ts.createSourceFile(file, code, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const found: string[] = [];
  const visit = (node: ts.Node) => {
    if (ts.isJsxText(node)) {
      const text = node.getText(source).trim();
      // Symbole wie "·", "×", "…" oder Ziffern allein sind keine Sprache
      if (/\p{L}/u.test(text) && !ALLOWED_TEXT.has(text)) found.push(text);
    } else if (
      ts.isJsxAttribute(node) &&
      TEXT_ATTRIBUTES.has(node.name.getText(source)) &&
      node.initializer &&
      ts.isStringLiteral(node.initializer) &&
      /\p{L}/u.test(node.initializer.text)
    ) {
      found.push(`${node.name.getText(source)}="${node.initializer.text}"`);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  return found;
}

describe("Oberflächentexte", () => {
  it("stehen in keiner Komponente fest im JSX (alles über t())", () => {
    const offenders = sourceFiles(SRC, /\.tsx$/).flatMap((f) => literalTexts(f).map((x) => `${f}: ${x}`));
    expect(offenders).toEqual([]);
  });

  it("erkennt feste Texte im JSX, aber keine Symbole und keine t()-Aufrufe (Selbsttest der Wache)", () => {
    const code = `const a = <p title="Hallo" data-x="x">Hallo Welt <b>·</b> <i>12</i>{t("x")}<img alt="Bild" /></p>;`;
    expect(literalTexts("probe.tsx", code)).toEqual(['title="Hallo"', "Hallo Welt", 'alt="Bild"']);
  });
});
