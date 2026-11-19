// Typen der Aufgaben (spiegeln `src-tauri/src/tasks/types.rs`, JSON in camelCase).
export type View = "list" | "today" | "planned" | "overdue" | "all" | "done";
export type RepeatKind = "daily" | "weekly" | "monthly" | "yearly";
export type Priority = 0 | 1 | 2 | 3;

/** `monthDay` (1–31): fester Tag im Monat für monatlich/jährlich; kurze Monate nehmen den letzten Tag, der Anker bleibt. */
export type RepeatRule = { kind: RepeatKind; interval: number; weekdays: number[]; fromDone: boolean; monthDay?: number | null };

export type TaskList = { id: string; name: string; color: string; position: number; ownerId: string | null };

export type Section = { id: string; listId: string; name: string; position: number };

export type Task = {
  id: string;
  listId: string;
  parentId: string | null;
  /** null = „Ohne Abschnitt“; Unteraufgaben haben keinen eigenen Abschnitt. */
  sectionId: string | null;
  title: string;
  notes: string;
  /** `YYYY-MM-DD` oder UTC-Sekunden als Text (bei `dueHasTime`). */
  due: string | null;
  dueHasTime: boolean;
  priority: Priority;
  doneAt: number | null;
  position: number;
  repeat: RepeatRule | null;
  remindMinutes: number | null;
  /** Für die spätere Zusammenarbeit vorbereitet, heute immer null. */
  assigneeId: string | null;
  createdBy: string | null;
};

/** Änderungen: Feldnamen wie im Kern (snake_case); nur genannte Felder werden geändert. */
export type TaskPatch = Partial<{
  title: string;
  notes: string;
  due: string | null;
  due_has_time: boolean;
  priority: Priority;
  repeat_rule: RepeatRule | null;
  remind_minutes: number | null;
  /** Nur beim Anlegen; später über `moveTask`. */
  section_id: string | null;
}>;

export type ViewCount = { view: View; count: number };

export type TaskReminder = { taskId: string; title: string; fireAt: number; minutesBefore: number; dueHasTime: boolean };

export type Unlisten = () => void;
