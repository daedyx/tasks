import { useCallback, useMemo, useState, type CSSProperties } from "react";
import { t } from "../i18n";
import { Notice } from "../ui/Notice";
import { ResizeHandle } from "../ui/ResizeHandle";
import { usePaneWidth } from "../ui/usePaneWidth";
import { dayText } from "./dueDay";
import { taskErrorText, toTaskError } from "./errors";
import * as api from "./api";
import { parseQuickTask } from "./quickEntry";
import { useTaskSearchRequest } from "./searchRequest";
import { positionBefore, type SortMode } from "./sorting";
import { TaskBoard } from "./TaskBoard";
import { TaskConfirm } from "./TaskConfirm";
import { TaskDetail } from "./TaskDetail";
import { TaskListView } from "./TaskListView";
import { TaskSidebar, type Selection } from "./TaskSidebar";
import type { TaskList, TaskPatch } from "./types";
import { useBoardMode } from "./useBoardMode";
import { useSections, useTaskLists, useTaskSearch, useTaskView, useViewCounts } from "./useTasksData";
import { ViewSwitch } from "./ViewSwitch";
import "../ui/toolbar.css";
import "./tasks.css";

export function TasksArea() {
  const lists = useTaskLists();
  const counts = useViewCounts();
  const [selection, setSelection] = useState<Selection>({ kind: "view", view: "today" });
  const [selected, setSelected] = useState<{ id: string; listId: string } | null>(null);
  const [sort, setSort] = useState<SortMode>("hand");
  const [query, setQuery] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [deleting, setDeleting] = useState<TaskList | null>(null);
  const side = usePaneWidth("daedyx.pane.tasksSide", 220, 180, 600);

  useTaskSearchRequest(useCallback((q: string) => setQuery(q), []));

  const view = useTaskView(selection.kind === "list" ? "list" : selection.view, selection.kind === "list" ? selection.id : null);
  const found = useTaskSearch(query);
  const fullList = useTaskView("list", selected?.listId ?? null);
  const ownListId = selection.kind === "list" && query === null ? selection.id : null;
  const [mode, setMode] = useBoardMode(ownListId);
  const boardSections = useSections(ownListId);
  const detailSections = useSections(selected?.listId ?? null);
  const switcher = ownListId ? <ViewSwitch mode={mode} onChange={setMode} /> : undefined;

  const shown = query !== null ? found.data : view.data;
  const task = selected ? (fullList.data.find((x) => x.id === selected.id) ?? shown.find((x) => x.id === selected.id) ?? null) : null;
  const subtasks = useMemo(() => fullList.data.filter((x) => x.parentId === selected?.id), [fullList.data, selected]);

  const fail = (e: unknown) => setError(taskErrorText(toTaskError(e).code));
  const run = (p: Promise<unknown>) => p.catch(fail);

  const title =
    query !== null ? t("tasks.search.title", { query }) : selection.kind === "list" ? (lists.data.find((l) => l.id === selection.id)?.name ?? t("tasks.view.list")) : t(`tasks.view.${selection.view}`);
  const targetList = selection.kind === "list" ? selection.id : (lists.data[0]?.id ?? null);

  const add = async (text: string) => {
    const quick = parseQuickTask(text, new Date());
    if (!quick || !targetList) return;
    const fields: TaskPatch = { ...quick.fields };
    if (selection.kind === "view" && selection.view === "today" && !fields.due) Object.assign(fields, { due: dayText(new Date()), due_has_time: false });
    try {
      await api.createTask(targetList, fields);
    } catch (e) {
      fail(e);
      throw e;
    }
  };

  const select = (s: Selection) => {
    setQuery(null);
    setSelection(s);
    setSelected(null);
  };

  const emptyText = lists.loaded && lists.data.length === 0 ? t("tasks.empty.none") : selection.kind === "list" && query === null ? t("tasks.empty.list") : query !== null ? t("tasks.search.none") : t("tasks.empty.view");

  const reorder = (id: string, beforeId: string | null) => {
    const moved = shown.find((x) => x.id === id);
    const before = beforeId ? shown.find((x) => x.id === beforeId) : null;
    if (!moved || (before && before.parentId !== moved.parentId)) return;
    const siblings = shown.filter((x) => x.listId === moved.listId && x.parentId === moved.parentId && x.id !== id);
    void run(api.moveTask(id, moved.listId, moved.parentId, moved.sectionId, positionBefore(siblings, beforeId)));
  };

  const confirmDelete = () => {
    if (!deleting) return;
    const gone = deleting.id;
    api.deleteTaskList(gone).then(
      () => {
        if (selection.kind === "list" && selection.id === gone) select({ kind: "view", view: "today" });
        setDeleting(null);
      },
      (e) => (fail(e), setDeleting(null)),
    );
  };

  return (
    <div className="tasks" style={{ "--side-w": `${side.width}px`, "--detail-w": task ? "340px" : "0px" } as CSSProperties}>
      <TaskSidebar
        lists={lists.data}
        counts={counts.data}
        selection={selection}
        onSelect={select}
        onCreate={async (name) => {
          try {
            const l = await api.createTaskList(name);
            select({ kind: "list", id: l.id });
          } catch (e) {
            fail(e);
          }
        }}
        onRename={(id, name) => void run(api.updateTaskList(id, { name }))}
        onDelete={setDeleting}
        onReorder={(ids) => void run(api.reorderTaskLists(ids))}
        // ans Ende der Zielliste (Zeitstempel als wachsende Position)
        onDropTask={(taskId, listId) => {
          const x = shown.find((s) => s.id === taskId);
          void run(api.moveTask(taskId, listId, null, x && x.listId === listId ? x.sectionId : null, Date.now()));
        }}
      />
      <ResizeHandle label={t("pane.resize_tasks")} width={side.width} min={side.min} max={side.max} onChange={side.setWidth} onReset={side.reset} />
      <main className="tasks__main">
        {error && <Notice onClose={() => setError(null)}>{error}</Notice>}
        {ownListId && mode === "board" ? (
          <TaskBoard
            listId={ownListId}
            title={title}
            tasks={shown}
            sections={boardSections.data}
            selectedId={selected?.id ?? null}
            headExtra={switcher}
            onSelect={(id) => setSelected({ id, listId: ownListId })}
            onError={fail}
          />
        ) : (
          <TaskListView
            headExtra={switcher}
            title={title}
            tasks={shown}
            sort={sort}
            onSort={setSort}
            selectedId={selected?.id ?? null}
            emptyText={emptyText}
            canAdd={targetList !== null && query === null}
            onAdd={add}
            onSelect={(id) => {
              const x = shown.find((s) => s.id === id);
              if (x) setSelected({ id, listId: x.listId });
            }}
            onToggle={(id) => void run(api.toggleTask(id))}
            onReorder={reorder}
            onClearSearch={query !== null ? () => setQuery(null) : undefined}
          />
        )}
      </main>
      {task && selected && (
        <TaskDetail
          key={task.id}
          task={task}
          subtasks={subtasks}
          sections={detailSections.data.filter((s) => s.listId === task.listId)}
          onMoveSection={(sectionId) => void run(api.moveTask(task.id, task.listId, task.parentId, sectionId, task.position))}
          onPatch={(p) => void run(api.updateTask(task.id, p))}
          onToggle={(id) => void run(api.toggleTask(id))}
          onAddSubtask={async (text) => {
            try {
              await api.createTask(task.listId, { title: text }, task.id);
            } catch (e) {
              fail(e);
              throw e;
            }
          }}
          onDelete={() => void api.deleteTask(task.id).then(() => setSelected(null), fail)}
          onClose={() => setSelected(null)}
        />
      )}
      {deleting && <TaskConfirm name={deleting.name} busy={false} error={null} onConfirm={confirmDelete} onCancel={() => setDeleting(null)} />}
    </div>
  );
}
