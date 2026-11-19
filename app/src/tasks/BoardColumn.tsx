import { useState, type KeyboardEvent, type MouseEvent } from "react";
import { t } from "../i18n";
import { ContextMenu } from "../ui/ContextMenu";
import { BoardAddField } from "./BoardAddField";
import { BoardCard } from "./BoardCard";
import type { Column } from "./boardModel";
import type { BoardDnd } from "./useBoardDnd";

type Props = {
  column: Column;
  progress: Map<string, [number, number]>;
  selectedId: string | null;
  dnd: BoardDnd;
  onSelect: (id: string) => void;
  onToggle: (id: string) => void;
  onAddCard: (sectionId: string | null, text: string) => Promise<void>;
  onRename: (sectionId: string, name: string) => void;
  onDelete: (sectionId: string) => void;
};

/** Eine Spalte (Abschnitt) mit Kopf, Karten und „+ Karte“. „Ohne Abschnitt“ lässt sich weder umbenennen noch löschen. */
export function BoardColumn({ column, progress, selectedId, dnd, onSelect, onToggle, onAddCard, onRename, onDelete }: Props) {
  const section = column.section;
  const name = section?.name ?? t("tasks.board.no_section");
  const [menu, setMenu] = useState<{ x: number; y: number } | null>(null);
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(name);

  const startRename = () => {
    setDraft(name);
    setEditing(true);
  };
  const finish = (save: boolean) => {
    setEditing(false);
    if (save && section && draft.trim() && draft.trim() !== name) onRename(section.id, draft.trim());
  };
  const onEditKey = (e: KeyboardEvent) => {
    if (e.key === "Enter") finish(true);
    else if (e.key === "Escape") {
      e.stopPropagation();
      finish(false);
    }
  };
  const openMenu = (e: MouseEvent) => {
    if (!section) return;
    e.preventDefault();
    setMenu({ x: e.clientX, y: e.clientY });
  };
  const openMenuAtButton = (e: MouseEvent<HTMLButtonElement>) => {
    const r = e.currentTarget.getBoundingClientRect();
    setMenu({ x: r.left, y: r.bottom });
  };

  return (
    <section
      className="boardcol"
      aria-label={name}
      data-over={dnd.over === `col:${column.id ?? ""}` || undefined}
      data-dragging={(section && dnd.column === section.id) || undefined}
      {...dnd.columnProps(column.id)}
    >
      <header className="boardcol__head" {...(section ? dnd.headProps(section.id) : {})} onContextMenu={openMenu}>
        {editing ? (
          <input
            className="boardcol__edit"
            type="text"
            autoFocus
            aria-label={t("tasks.section.name_label")}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onKeyDown={onEditKey}
            onBlur={() => finish(true)}
          />
        ) : (
          <h3 className="boardcol__name" onDoubleClick={section ? startRename : undefined}>
            {name}
          </h3>
        )}
        <span className="boardcol__count">{column.cards.length}</span>
        {section && !editing && (
          <button type="button" className="boardcol__menu" aria-label={t("tasks.section.menu_open", { name })} aria-haspopup="menu" onClick={openMenuAtButton}>
            ⋯
          </button>
        )}
      </header>
      <ul className="boardcol__cards">
        {column.cards.map((task) => (
          <BoardCard
            key={task.id}
            task={task}
            progress={progress.get(task.id) ?? null}
            selected={task.id === selectedId}
            dragging={dnd.card === task.id}
            over={dnd.over === `card:${task.id}`}
            onSelect={() => onSelect(task.id)}
            onToggle={() => onToggle(task.id)}
            dnd={dnd.cardProps(task.id, column.id)}
          />
        ))}
        {column.cards.length === 0 && <li className="boardcol__empty">{t("tasks.board.empty_column")}</li>}
      </ul>
      <BoardAddField
        openLabel={t("tasks.board.add_card")}
        fieldLabel={t("tasks.board.card_label", { name })}
        placeholder={t("tasks.board.card_placeholder")}
        submitLabel={t("tasks.board.add_card_submit")}
        cancelLabel={t("tasks.dialog.cancel")}
        onSubmit={(text) => onAddCard(column.id, text)}
      />
      {menu && section && (
        <ContextMenu
          x={menu.x}
          y={menu.y}
          label={t("tasks.section.menu", { name })}
          onClose={() => setMenu(null)}
          items={[
            { key: "rename", label: t("tasks.section.rename"), onSelect: startRename },
            { key: "delete", label: t("tasks.section.delete"), onSelect: () => onDelete(section.id) },
          ]}
        />
      )}
    </section>
  );
}
