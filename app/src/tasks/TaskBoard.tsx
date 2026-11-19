import { useMemo, useState, type ReactNode } from "react";
import { t } from "../i18n";
import * as api from "./api";
import { BoardAddField } from "./BoardAddField";
import { BoardColumn } from "./BoardColumn";
import { boardColumns, reorderedSectionIds, subtaskProgress } from "./boardModel";
import { parseQuickTask } from "./quickEntry";
import { positionBefore } from "./sorting";
import { TaskConfirm } from "./TaskConfirm";
import type { Section, Task, TaskPatch } from "./types";
import { useBoardDnd } from "./useBoardDnd";
import "./board.css";

type Props = {
  listId: string;
  title: string;
  tasks: Task[];
  sections: Section[];
  selectedId: string | null;
  headExtra: ReactNode;
  onSelect: (id: string) => void;
  /** Fehler aus Befehlen (zeigt der Bereich als Hinweis) */
  onError: (e: unknown) => void;
};

/** Board einer eigenen Liste: Abschnitte als Spalten, Karten per Ziehen verschieben. */
export function TaskBoard({ listId, title, tasks, sections, selectedId, headExtra, onSelect, onError }: Props) {
  const own = useMemo(() => sections.filter((s) => s.listId === listId), [sections, listId]);
  const columns = useMemo(() => boardColumns(tasks, own), [tasks, own]);
  const progress = useMemo(() => subtaskProgress(tasks), [tasks]);
  const [deleting, setDeleting] = useState<Section | null>(null);
  const run = (p: Promise<unknown>) => p.catch(onError);

  const moveCard = (taskId: string, sectionId: string | null, beforeId: string | null) => {
    const col = columns.find((c) => c.id === sectionId);
    const moved = tasks.find((x) => x.id === taskId);
    if (!col || !moved || moved.parentId) return;
    const siblings = col.cards.filter((x) => x.id !== taskId);
    void run(api.moveTask(taskId, listId, null, sectionId, positionBefore(siblings, beforeId)));
  };
  const dnd = useBoardDnd({
    onMoveCard: moveCard,
    onMoveColumn: (id, beforeId) => void run(api.reorderSections(listId, reorderedSectionIds(own, id, beforeId))),
  });

  const addCard = async (sectionId: string | null, text: string) => {
    const quick = parseQuickTask(text, new Date());
    if (!quick) return;
    const fields: TaskPatch = { ...quick.fields, ...(sectionId ? { section_id: sectionId } : {}) };
    try {
      await api.createTask(listId, fields);
    } catch (e) {
      onError(e);
      throw e;
    }
  };
  const addSection = async (name: string) => {
    try {
      await api.createSection(listId, name);
    } catch (e) {
      onError(e);
      throw e;
    }
  };

  return (
    <section className="board" aria-label={t("tasks.board.label", { title })}>
      <header className="tasklist__head">
        <h2 className="tasklist__title">{title}</h2>
        {headExtra}
      </header>
      <div className="board__cols">
        {columns.map((c) => (
          <BoardColumn
            key={c.id ?? "none"}
            column={c}
            progress={progress}
            selectedId={selectedId}
            dnd={dnd}
            onSelect={onSelect}
            onToggle={(id) => void run(api.toggleTask(id))}
            onAddCard={addCard}
            onRename={(id, name) => void run(api.renameSection(id, name))}
            onDelete={(id) => setDeleting(own.find((s) => s.id === id) ?? null)}
          />
        ))}
        <div className="boardtail" data-over={dnd.over === "col:tail" || undefined} {...dnd.tailProps}>
          <BoardAddField
            openLabel={t("tasks.board.add_section")}
            fieldLabel={t("tasks.board.section_label")}
            placeholder={t("tasks.board.section_placeholder")}
            submitLabel={t("tasks.board.add_card_submit")}
            cancelLabel={t("tasks.dialog.cancel")}
            onSubmit={addSection}
          />
        </div>
      </div>
      {deleting && (
        <TaskConfirm
          kind="section"
          name={deleting.name}
          busy={false}
          error={null}
          onCancel={() => setDeleting(null)}
          onConfirm={() => {
            const id = deleting.id;
            setDeleting(null);
            void run(api.deleteSection(id));
          }}
        />
      )}
    </section>
  );
}
