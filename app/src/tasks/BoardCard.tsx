import type { DragEvent } from "react";
import { t } from "../i18n";
import { dueLabel } from "./dueLabel";
import type { Task } from "./types";

type Props = {
  task: Task;
  /** (erledigt, gesamt) der Unteraufgaben; null ohne Unteraufgaben */
  progress: [number, number] | null;
  selected: boolean;
  dragging: boolean;
  /** Ablage-Hinweis: eine gezogene Karte landet vor dieser */
  over: boolean;
  onSelect: () => void;
  onToggle: () => void;
  dnd: { draggable: true; onDragStart: (e: DragEvent) => void; onDragOver: (e: DragEvent) => void; onDrop: (e: DragEvent) => void; onDragEnd: () => void };
};

/** Karte im Board: Abhaken, Titel, Fälligkeit, Priorität, Zähler der Unteraufgaben. Abschnitt und „erledigt“ sind unabhängig. */
export function BoardCard({ task, progress, selected, dragging, over, onSelect, onToggle, dnd }: Props) {
  const done = task.doneAt !== null;
  const due = dueLabel(task, new Date());
  return (
    <li {...dnd} className="boardcard" data-selected={selected || undefined} data-done={done || undefined} data-dragging={dragging || undefined} data-over={over || undefined}>
      <button
        type="button"
        role="checkbox"
        aria-checked={done}
        aria-label={t(done ? "tasks.row.reopen" : "tasks.row.toggle", { title: task.title })}
        className="taskrow__check"
        data-priority={task.priority}
        onClick={onToggle}
      >
        {done && <span aria-hidden="true">✓</span>}
      </button>
      <button type="button" className="boardcard__main" aria-label={t("tasks.row.open", { title: task.title })} onClick={onSelect}>
        <span className="boardcard__title">{task.title}</span>
        <span className="taskrow__meta">
          {due && (
            <span className="taskrow__due" data-overdue={due.overdue || undefined}>
              {due.text}
            </span>
          )}
          {task.repeat && <span title={t("tasks.row.repeats")}>↻</span>}
          {task.priority > 0 && (
            <span className="taskrow__prio" data-priority={task.priority} title={t(`tasks.priority.${task.priority}`)}>
              {"!".repeat(task.priority)}
            </span>
          )}
          {progress && <span>{t("tasks.row.subtasks", { done: progress[0], total: progress[1] })}</span>}
        </span>
      </button>
    </li>
  );
}
