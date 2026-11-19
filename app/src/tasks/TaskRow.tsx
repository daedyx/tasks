import type { CSSProperties, HTMLAttributes } from "react";
import { t } from "../i18n";
import { dueLabel } from "./dueLabel";
import type { Task } from "./types";

type Props = {
  task: Task;
  selected: boolean;
  /** Ebene der Aufgabe in der Ansicht (0 = oberste); tiefere Ebenen sind weiter eingerückt */
  depth: number;
  /** (erledigt, gesamt) der Unteraufgaben; null ohne Unteraufgaben */
  progress: [number, number] | null;
  onSelect: () => void;
  onToggle: () => void;
  /** Ziehen zum Sortieren (HTML5), nur in der Handreihenfolge */
  dnd?: Omit<HTMLAttributes<HTMLLIElement>, "className"> & { draggable?: boolean };
};

/** Eine Aufgabenzeile: Abhaken, Titel, Fälligkeit, Priorität, Platz für den späteren Zuständigen. */
export function TaskRow({ task, selected, depth, progress, onSelect, onToggle, dnd }: Props) {
  const done = task.doneAt !== null;
  const due = dueLabel(task, new Date());
  return (
    <li {...dnd} className="taskrow" data-selected={selected || undefined} data-done={done || undefined} data-depth={depth > 0 ? depth : undefined} style={depth > 0 ? ({ "--depth": depth } as CSSProperties) : undefined}>
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
      <button type="button" className="taskrow__main" aria-label={t("tasks.row.open", { title: task.title })} onClick={onSelect}>
        <span className="taskrow__title">{task.title}</span>
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
      {/* Platz für den Avatar des Zuständigen (Zusammenarbeit); bleibt leer, ohne dass sich das Layout verschiebt */}
      <span className="taskrow__assignee" data-assigned={task.assigneeId ? "" : undefined} aria-hidden="true">
        {task.assigneeId ? task.assigneeId.slice(0, 2).toUpperCase() : ""}
      </span>
    </li>
  );
}
