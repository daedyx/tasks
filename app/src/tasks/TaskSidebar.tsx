import { useEffect, useRef, useState, type DragEvent, type MouseEvent } from "react";
import { t } from "../i18n";
import { ContextMenu } from "../ui/ContextMenu";
import type { TaskList, View, ViewCount } from "./types";

export type Selection = { kind: "view"; view: Exclude<View, "list"> } | { kind: "list"; id: string };

const SMART: Exclude<View, "list">[] = ["today", "planned", "overdue", "all", "done"];

type Props = {
  lists: TaskList[];
  counts: ViewCount[];
  selection: Selection;
  onSelect: (s: Selection) => void;
  onCreate: (name: string) => Promise<void>;
  onRename: (id: string, name: string) => void;
  onDelete: (list: TaskList) => void;
  onReorder: (ids: string[]) => void;
  /** Aufgabe per Ziehen in eine andere Liste legen */
  onDropTask: (taskId: string, listId: string) => void;
};

function NameInput({ initial, label, onDone }: { initial: string; label: string; onDone: (name: string | null) => void }) {
  const [draft, setDraft] = useState(initial);
  const ref = useRef<HTMLInputElement>(null);
  const done = useRef(false);
  useEffect(() => {
    ref.current?.focus();
    ref.current?.select();
  }, []);
  const finish = (v: string | null) => {
    if (done.current) return;
    done.current = true;
    onDone(v);
  };
  return (
    <input
      ref={ref}
      className="taskside__input"
      aria-label={label}
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={() => finish(draft.trim() || null)}
      onKeyDown={(e) => {
        if (e.key === "Enter") finish(draft.trim() || null);
        if (e.key === "Escape") finish(null);
      }}
    />
  );
}

export function TaskSidebar({ lists, counts, selection, onSelect, onCreate, onRename, onDelete, onReorder, onDropTask }: Props) {
  const [menu, setMenu] = useState<{ x: number; y: number; list: TaskList } | null>(null);
  const [renaming, setRenaming] = useState<string | null>(null);
  const [adding, setAdding] = useState(false);
  const [dragId, setDragId] = useState<string | null>(null);
  const count = (v: View) => counts.find((c) => c.view === v)?.count;

  const drop = (e: DragEvent, targetId: string) => {
    e.preventDefault();
    const taskId = e.dataTransfer.getData("application/x-daedyx-task");
    if (taskId) {
      onDropTask(taskId, targetId);
      return;
    }
    if (!dragId || dragId === targetId) return;
    const ids = lists.map((l) => l.id).filter((id) => id !== dragId);
    ids.splice(ids.indexOf(targetId), 0, dragId);
    onReorder(ids);
    setDragId(null);
  };

  return (
    <nav className="taskside" aria-label={t("tasks.sidebar.label")}>
      <h3 className="taskside__heading">{t("tasks.sidebar.smart")}</h3>
      <ul className="taskside__list">
        {SMART.map((v) => {
          const n = count(v);
          const on = selection.kind === "view" && selection.view === v;
          return (
            <li key={v}>
              <button type="button" className="taskside__item" aria-current={on ? "true" : undefined} onClick={() => onSelect({ kind: "view", view: v })}>
                <span>{t(`tasks.view.${v}`)}</span>
                {n !== undefined && n > 0 && v !== "done" && <span className="taskside__count" aria-label={t("tasks.sidebar.count", { n })}>{n}</span>}
              </button>
            </li>
          );
        })}
      </ul>
      <h3 className="taskside__heading">{t("tasks.sidebar.lists")}</h3>
      {lists.length === 0 && !adding && <p className="taskside__hint">{t("tasks.sidebar.no_lists")}</p>}
      <ul className="taskside__list">
        {lists.map((l) => {
          const on = selection.kind === "list" && selection.id === l.id;
          return (
            <li
              key={l.id}
              draggable={renaming !== l.id}
              onDragStart={() => setDragId(l.id)}
              onDragOver={(e) => (dragId || e.dataTransfer.types.includes("application/x-daedyx-task")) && e.preventDefault()}
              onDrop={(e) => drop(e, l.id)}
              onDragEnd={() => setDragId(null)}
              data-dragging={dragId === l.id || undefined}
            >
              {renaming === l.id ? (
                <NameInput
                  initial={l.name}
                  label={t("tasks.sidebar.rename_label", { name: l.name })}
                  onDone={(n) => {
                    setRenaming(null);
                    if (n && n !== l.name) onRename(l.id, n);
                  }}
                />
              ) : (
                <button
                  type="button"
                  className="taskside__item"
                  aria-current={on ? "true" : undefined}
                  onClick={() => onSelect({ kind: "list", id: l.id })}
                  onContextMenu={(e: MouseEvent) => {
                    e.preventDefault();
                    setMenu({ x: e.clientX, y: e.clientY, list: l });
                  }}
                >
                  <span className="taskside__dot" aria-hidden="true" />
                  <span>{l.name}</span>
                  {/* Platz für das spätere Freigabe-Symbol (Zusammenarbeit) */}
                  <span className="taskside__share" aria-hidden="true" data-shared={l.ownerId ? "" : undefined} />
                </button>
              )}
            </li>
          );
        })}
      </ul>
      {adding ? (
        <NameInput
          initial=""
          label={t("tasks.sidebar.new_list_name")}
          onDone={(n) => {
            setAdding(false);
            if (n) void onCreate(n).catch(() => undefined);
          }}
        />
      ) : (
        <button type="button" className="taskside__new" onClick={() => setAdding(true)}>
          + {t("tasks.sidebar.new_list")}
        </button>
      )}
      {menu && (
        <ContextMenu
          x={menu.x}
          y={menu.y}
          label={menu.list.name}
          onClose={() => setMenu(null)}
          items={[
            { key: "rename", label: t("tasks.menu.rename"), onSelect: () => (setRenaming(menu.list.id), setMenu(null)) },
            { key: "delete", label: t("tasks.menu.delete"), onSelect: () => (onDelete(menu.list), setMenu(null)) },
          ]}
        />
      )}
    </nav>
  );
}
