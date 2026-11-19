import de from "./de.json";

const dict: Record<string, string> = de;

export function t(key: string, params?: Record<string, string | number>): string {
  const text = dict[key];
  if (text === undefined) return key;
  if (!params) return text;
  return text.replace(/\{(\w+)\}/g, (_, name: string) => String(params[name] ?? `{${name}}`));
}
