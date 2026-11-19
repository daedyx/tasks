import { useMemo, useState, type FormEvent, type ReactNode } from "react";
import { t } from "../i18n";
import { subtaskProgress, taskDepths } from "./boardModel";
import { sortTasks, SORT_MODES, type SortMode } from "./sorting";
import { TaskRow } from "./TaskRow";
import type { Task } from "./types";

type Props = {
  title: string;
  tasks: Task[];
  sort: SortMode;
  onSort: (m: SortMode) => void;
  selectedId: string | null;
  emptyText: string;
  /** Schnelleingabe nur, wenn eine Liste gewählt ist oder es eine Standardliste gibt */
  canAdd: boolean;
  onAdd: (text: string) => Promise<void>;
  onSelect: (id: string) => void;
  onToggle: (id: string) => void;
  /** Aufgabe `id` direkt vor `beforeId` einsortieren (null = ans Ende); nur in der Handreihenfolge angeboten */
  onReorder: (id: string, beforeId: string | null) => void;
  /** Bei Suchergebnissen: Titelzeile mit „Suche beenden“ */
  onClearSearch?: () => void;
  /** Platz in der Kopfzeile, z. B. der Umschalter Liste/Board */
  headExtra?: ReactNode;
};

export function TaskListView({ title, tasks, sort, onSort, selectedId, emptyText, canAdd, onAdd, onSelect, onToggle, onReorder, onClearSearch, headExtra }: Props) {
  const [draft, setDraft] = useState("");
  const [busy, setBusy] = useState(false);
  const [dragId, setDragId] = useState<string | null>(null);
  const sorted = useMemo(() => sortTasks(tasks, sort), [tasks, sort]);
  const progress = useMemo(() => subtaskProgress(tasks), [tasks]);
  const depths = useMemo(() => taskDepths(tasks), [tasks]);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    if (!draft.trim() || busy) return;
    setBusy(true);
    onAdd(draft)
      .then(() => setDraft(""))
      .catch(() => undefined)
      .finally(() => setBusy(false));
  };

  return (
    <section className="tasklist" aria-label={title}>
      <header className="tasklist__head">
        <h2 className="tasklist__title">{title}</h2>
        {onClearSearch && (
          <button type="button" className="tasks__btn" onClick={onClearSearch}>
            {t("tasks.search.clear")}
          </button>
        )}
        {headExtra}
        <label className="tasklist__sort">
          <span>{t("tasks.sort.label")}</span>
          <select value={sort} onChange={(e) => onSort(e.target.value as SortMode)}>
            {SORT_MODES.map((m) => (
              <option key={m} value={m}>
                {t(`tasks.sort.${m}`)}
              </option>
            ))}
          </select>
        </label>
      </header>
      {canAdd && (
        <form className="tasklist__add" onSubmit={submit}>
          <input
            type="text"
            aria-label={t("tasks.add.label")}
            placeholder={t("tasks.add.placeholder")}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
          />
          <button type="submit" className="tasks__btn tasks__btn--primary" disabled={!draft.trim() || busy}>
            {t("tasks.add.button")}
          </button>
        </form>
      )}
      {sorted.length === 0 ? (
        <p className="tasklist__empty">{emptyText}</p>
      ) : (
        <ul className="tasklist__rows">
          {sorted.map((task) => (
            <TaskRow
              key={task.id}
              task={task}
              depth={depths.get(task.id) ?? 0}
              progress={progress.get(task.id) ?? null}
              selected={task.id === selectedId}
              onSelect={() => onSelect(task.id)}
              onToggle={() => onToggle(task.id)}
              dnd={
                sort === "hand"
                  ? {
                      draggable: true,
                      onDragStart: (e) => {
                        e.dataTransfer.setData("application/x-daedyx-task", task.id);
                        e.dataTransfer.effectAllowed = "move";
                        setDragId(task.id);
                      },
                      onDragOver: (e) => dragId && dragId !== task.id && e.preventDefault(),
                      onDrop: (e) => {
                        e.preventDefault();
                        if (dragId && dragId !== task.id) onReorder(dragId, task.id);
                        setDragId(null);
                      },
                      onDragEnd: () => setDragId(null),
                    }
                  : undefined
              }
            />
          ))}
        </ul>
      )}
    </section>
  );
}
