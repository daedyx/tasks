// Schnelleingabe für Aufgaben: „Steuer morgen 18 Uhr !!“ → Titel, Fälligkeit, Priorität. Reine Funktion.
// Datum und Uhrzeit liest die Schnelleingabe des Kalenders; `!`, `!!`, `!!!` setzen die Priorität.
import { parseQuickEntry } from "./dateEntry";
import { dayText } from "./dueDay";
import type { Priority, TaskPatch } from "./types";

export type QuickTask = { title: string; fields: TaskPatch };

export function parseQuickTask(input: string, now: Date): QuickTask | null {
  let priority: Priority = 0;
  const rest = ` ${input.trim()} `.replace(/\s(!{1,3})(?=\s)/, (_, bangs: string) => {
    priority = bangs.length as Priority;
    return " ";
  });
  const parsed = parseQuickEntry(rest.trim(), now);
  const fields: TaskPatch = {};
  if (priority) fields.priority = priority;
  if (!parsed || !parsed.title.trim()) {
    const title = rest.trim();
    return title ? { title, fields: { ...fields, title } } : null;
  }
  fields.title = parsed.title.trim();
  if (parsed.hasWhen) {
    if (parsed.allDay) {
      fields.due = dayText(parsed.start);
      fields.due_has_time = false;
    } else {
      fields.due = String(Math.floor(parsed.start.getTime() / 1000));
      fields.due_has_time = true;
    }
  }
  return { title: fields.title, fields };
}
