import { useState, type FormEvent } from "react";
import { t } from "../i18n";
import { dueDay, dueTime, makeDue } from "./dueDay";
import { RepeatFields } from "./RepeatFields";
import { useDebouncedField } from "./useDebouncedField";
import type { Priority, Section, Task, TaskPatch } from "./types";

const REMINDERS: [number, string][] = [
  [0, "tasks.reminder.at_due"],
  [5, "tasks.reminder.min_5"],
  [30, "tasks.reminder.min_30"],
  [60, "tasks.reminder.hour_1"],
  [1440, "tasks.reminder.day_1"],
];

type Props = {
  task: Task;
  subtasks: Task[];
  /** Abschnitte der Liste der Aufgabe (leer = keine Auswahl) */
  sections: Section[];
  onPatch: (patch: TaskPatch) => void;
  onMoveSection: (sectionId: string | null) => void;
  onToggle: (id: string) => void;
  onAddSubtask: (title: string) => Promise<void>;
  onDelete: () => void;
  onClose: () => void;
};

/** Einzelheiten einer Aufgabe. Texte werden beim Verlassen des Feldes gespeichert, alles andere sofort. */
export function TaskDetail({ task, subtasks, sections, onPatch, onMoveSection, onToggle, onAddSubtask, onDelete, onClose }: Props) {
  const [title, setTitle] = useState(task.title);
  const [notes, setNotes] = useState(task.notes);
  const [sub, setSub] = useState("");
  const day = dueDay(task) ?? "";
  const time = dueTime(task) ?? "";
  const dayField = useDebouncedField(day, (v) => onPatch(v ? makeDue(v, time) : { ...makeDue("", ""), remind_minutes: null }));
  const timeField = useDebouncedField(time, (v) => onPatch(makeDue(day, v)));

  const addSub = (e: FormEvent) => {
    e.preventDefault();
    if (!sub.trim()) return;
    onAddSubtask(sub)
      .then(() => setSub(""))
      .catch(() => undefined);
  };

  return (
    <aside className="taskdetail" aria-label={t("tasks.detail.label")}>
      <div className="taskdetail__bar">
        <button type="button" className="tasks__btn" onClick={onClose}>
          {t("tasks.detail.close")}
        </button>
      </div>
      <label className="taskdetail__field">
        <span>{t("tasks.detail.title")}</span>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onBlur={() => (title.trim() ? title.trim() !== task.title && onPatch({ title }) : setTitle(task.title))}
        />
      </label>
      <label className="taskdetail__field">
        <span>{t("tasks.detail.notes")}</span>
        <textarea rows={4} value={notes} onChange={(e) => setNotes(e.target.value)} onBlur={() => notes !== task.notes && onPatch({ notes })} />
      </label>
      <div className="taskdetail__row">
        <label className="taskdetail__field">
          <span>{t("tasks.detail.due")}</span>
          <input type="date" {...dayField} />
        </label>
        <label className="taskdetail__field">
          <span>{t("tasks.detail.time")}</span>
          <input type="time" {...timeField} disabled={!day} />
        </label>
      </div>
      {!task.parentId && sections.length > 0 && (
        <label className="taskdetail__field">
          <span>{t("tasks.detail.section")}</span>
          <select value={sections.some((s) => s.id === task.sectionId) ? (task.sectionId ?? "") : ""} onChange={(e) => onMoveSection(e.target.value || null)}>
            <option value="">{t("tasks.detail.section_none")}</option>
            {sections.map((s) => (
              <option key={s.id} value={s.id}>
                {s.name}
              </option>
            ))}
          </select>
        </label>
      )}
      <label className="taskdetail__field">
        <span>{t("tasks.detail.priority")}</span>
        <select value={task.priority} onChange={(e) => onPatch({ priority: Number(e.target.value) as Priority })}>
          {[0, 1, 2, 3].map((p) => (
            <option key={p} value={p}>
              {t(`tasks.priority.${p}`)}
            </option>
          ))}
        </select>
      </label>
      <label className="taskdetail__field">
        <span>{t("tasks.detail.reminder")}</span>
        <select
          value={task.remindMinutes ?? ""}
          disabled={!day}
          onChange={(e) => onPatch({ remind_minutes: e.target.value === "" ? null : Number(e.target.value) })}
        >
          <option value="">{t("tasks.detail.reminder_none")}</option>
          {REMINDERS.map(([m, key]) => (
            <option key={m} value={m}>
              {t(key)}
            </option>
          ))}
        </select>
        {!day && <small>{t("tasks.detail.reminder_needs_due")}</small>}
      </label>
      <RepeatFields value={task.repeat} onChange={(rule) => onPatch({ repeat_rule: rule })} />
      {/* Vorbereitet für die Zusammenarbeit, ohne Funktion */}
      <label className="taskdetail__field">
        <span>{t("tasks.assignee.label")}</span>
        <select disabled aria-describedby="assignee-soon">
          <option>{t("tasks.assignee.placeholder")}</option>
        </select>
        <small id="assignee-soon">{t("tasks.assignee.soon")}</small>
      </label>
      {!task.parentId && (
        <section className="taskdetail__group" aria-label={t("tasks.detail.subtasks")}>
          <h3>{t("tasks.detail.subtasks")}</h3>
          <ul className="taskdetail__subs">
            {subtasks.map((k) => (
              <li key={k.id}>
                <label>
                  <input type="checkbox" checked={k.doneAt !== null} onChange={() => onToggle(k.id)} />
                  <span data-done={k.doneAt !== null || undefined}>{k.title}</span>
                </label>
              </li>
            ))}
          </ul>
          <form className="taskdetail__addsub" onSubmit={addSub}>
            <input type="text" aria-label={t("tasks.detail.add_subtask")} placeholder={t("tasks.detail.subtask_placeholder")} value={sub} onChange={(e) => setSub(e.target.value)} />
            <button type="submit" className="tasks__btn" disabled={!sub.trim()}>
              {t("tasks.add.button")}
            </button>
          </form>
        </section>
      )}
      <button type="button" className="tasks__btn tasks__btn--danger" onClick={onDelete}>
        {t("tasks.detail.delete")}
      </button>
    </aside>
  );
}
