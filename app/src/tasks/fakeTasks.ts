// Fake-Kern für Aufgaben: nur im Entwicklungs- und Testbetrieb ohne Tauri (siehe `api.ts`), erfundene Daten.
import { dayText, dueDay } from "./dueDay";
import { nextDue } from "./repeatNext";
import type { Priority, Section, Task, TaskList, TaskPatch, View, ViewCount } from "./types";

export type FakeTasks = ReturnType<typeof createFakeTasks>;

const addDays = (d: Date, n: number): Date => new Date(d.getFullYear(), d.getMonth(), d.getDate() + n);

export function createFakeTasks(now: () => Date = () => new Date()) {
  let seq = 0;
  let listeners = new Set<() => void>();
  const notify = () => listeners.forEach((cb) => cb());
  const id = (p: string): string => `${p}-${++seq}`;
  let lists: TaskList[] = [];
  let tasks: Task[] = [];
  let sections: Section[] = [];

  const mkTask = (listId: string, title: string, extra: Partial<Task> = {}): Task => ({
    id: id("t"), listId, parentId: null, sectionId: null, title, notes: "", due: null, dueHasTime: false, priority: 0, doneAt: null,
    position: tasks.filter((t) => t.listId === listId && !t.parentId).length + 1, repeat: null, remindMinutes: null,
    assigneeId: null, createdBy: null, ...extra,
  });

  function seed() {
    const today = now();
    lists = [
      { id: "haushalt", name: "Haushalt", color: "", position: 1, ownerId: null },
      { id: "arbeit", name: "Arbeit", color: "", position: 2, ownerId: null },
    ];
    tasks = [];
    sections = [];
    tasks.push(mkTask("haushalt", "Altglas wegbringen", { due: dayText(today), priority: 2 }));
    tasks.push(mkTask("haushalt", "Pflanzen gießen", { due: dayText(addDays(today, -1)), repeat: { kind: "weekly", interval: 1, weekdays: [], fromDone: false } }));
    const shop = mkTask("haushalt", "Einkaufen", { due: dayText(addDays(today, 2)) });
    tasks.push(shop, mkTask("haushalt", "Milch", { parentId: shop.id }), mkTask("haushalt", "Brot", { parentId: shop.id }));
    tasks.push(mkTask("arbeit", "Bericht schreiben", { due: dayText(addDays(today, 5)), priority: 3 }));
    tasks.push(mkTask("arbeit", "Raum buchen"));
  }
  seed();

  const live = (t: Task) => lists.some((l) => l.id === t.listId);
  const byPos = (a: Task, b: Task) => a.position - b.position;
  const find = (taskId: string): Task => {
    const t = tasks.find((x) => x.id === taskId);
    if (!t) throw { code: "task_not_found", message: "" };
    return t;
  };

  function inList(listId: string): Task[] {
    const all = tasks.filter((t) => t.listId === listId).sort(byPos);
    return all.filter((t) => !t.parentId).flatMap((t) => [t, ...all.filter((k) => k.parentId === t.id)]);
  }

  function query(view: View, listId?: string | null): Task[] {
    const today = dayText(now());
    const open = tasks.filter((t) => live(t) && t.doneAt === null);
    const dated = (f: (d: string) => boolean) =>
      open.filter((t) => { const d = dueDay(t); return d !== null && f(d); })
        .sort((a, b) => (dueDay(a) ?? "").localeCompare(dueDay(b) ?? "") || b.priority - a.priority || a.position - b.position);
    switch (view) {
      case "list": if (!listId) throw { code: "invalid", message: "" }; return inList(listId);
      case "today": return dated((d) => d === today);
      case "overdue": return dated((d) => d < today);
      case "planned": return dated(() => true);
      case "all": return open.sort(byPos);
      case "done": return tasks.filter((t) => live(t) && t.doneAt !== null).sort((a, b) => (b.doneAt ?? 0) - (a.doneAt ?? 0));
    }
  }

  function patchTask(t: Task, p: TaskPatch): void {
    if ("title" in p) { const s = (p.title ?? "").trim(); if (!s) throw { code: "invalid", message: "" }; t.title = s; }
    if ("notes" in p) t.notes = p.notes ?? "";
    if ("due" in p) t.due = p.due ?? null;
    if ("due_has_time" in p) t.dueHasTime = !!p.due_has_time;
    if ("priority" in p) t.priority = (p.priority ?? 0) as Priority;
    if ("repeat_rule" in p) t.repeat = p.repeat_rule ?? null;
    if ("remind_minutes" in p) t.remindMinutes = p.remind_minutes ?? null;
    if ("section_id" in p) t.sectionId = p.section_id ?? null;
  }

  return {
    subscribe(cb: () => void): () => void { listeners.add(cb); return () => listeners.delete(cb); },
    reset() { seq = 0; listeners = new Set(); seed(); },
    lists: async (): Promise<TaskList[]> => [...lists].sort((a, b) => a.position - b.position),
    createList(name: string, color: string): TaskList {
      if (!name.trim()) throw { code: "invalid", message: "" };
      const l = { id: id("l"), name: name.trim(), color, position: lists.length + 1, ownerId: null };
      lists.push(l); notify(); return l;
    },
    updateList(listId: string, name?: string, color?: string): void {
      const l = lists.find((x) => x.id === listId);
      if (!l) throw { code: "list_not_found", message: "" };
      if (name !== undefined) l.name = name.trim() || l.name;
      if (color !== undefined) l.color = color;
      notify();
    },
    reorderLists(ids: string[]): void { ids.forEach((x, i) => { const l = lists.find((y) => y.id === x); if (l) l.position = i + 1; }); notify(); },
    deleteList(listId: string): void { lists = lists.filter((l) => l.id !== listId); tasks = tasks.filter((t) => t.listId !== listId); sections = sections.filter((s) => s.listId !== listId); notify(); },
    query,
    counts(): ViewCount[] {
      return (["today", "planned", "overdue", "all"] as View[]).map((view) => ({ view, count: query(view).length }));
    },
    search(text: string): Task[] {
      const q = text.trim().toLowerCase();
      return q ? tasks.filter((t) => live(t) && (t.title.toLowerCase().includes(q) || t.notes.toLowerCase().includes(q))).slice(0, 50) : [];
    },
    create(listId: string, parentId: string | null, fields: TaskPatch): Task {
      if (!lists.some((l) => l.id === listId)) throw { code: "list_not_found", message: "" };
      if (parentId && find(parentId).parentId) throw { code: "too_deep", message: "" };
      const t = mkTask(listId, "x", { parentId });
      patchTask(t, fields);
      t.position = Math.max(0, ...tasks.filter((x) => x.listId === listId && x.parentId === parentId).map((x) => x.position)) + 1;
      tasks.push(t); notify(); return t;
    },
    update(taskId: string, p: TaskPatch): Task { const t = find(taskId); patchTask(t, p); notify(); return t; },
    toggle(taskId: string): Task {
      const t = find(taskId);
      const day = dueDay(t);
      if (t.doneAt !== null) t.doneAt = null;
      else if (t.repeat && day) { t.due = t.dueHasTime ? t.due : nextDue(t.repeat, day); }
      else t.doneAt = Math.floor(now().getTime() / 1000);
      notify(); return t;
    },
    sections: async (listId: string): Promise<Section[]> => sections.filter((s) => s.listId === listId).sort((a, b) => a.position - b.position),
    createSection(listId: string, name: string): Section {
      if (!lists.some((l) => l.id === listId)) throw { code: "list_not_found", message: "" };
      if (!name.trim()) throw { code: "invalid", message: "" };
      const s = { id: id("s"), listId, name: name.trim(), position: Math.max(0, ...sections.filter((x) => x.listId === listId).map((x) => x.position)) + 1 };
      sections.push(s); notify(); return s;
    },
    renameSection(sectionId: string, name: string): void {
      const s = sections.find((x) => x.id === sectionId);
      if (!s) throw { code: "section_not_found", message: "" };
      if (!name.trim()) throw { code: "invalid", message: "" };
      s.name = name.trim(); notify();
    },
    reorderSections(listId: string, ids: string[]): void {
      ids.forEach((x, i) => { const s = sections.find((y) => y.id === x && y.listId === listId); if (s) s.position = i + 1; }); notify();
    },
    deleteSection(sectionId: string): void {
      if (!sections.some((s) => s.id === sectionId)) throw { code: "section_not_found", message: "" };
      sections = sections.filter((s) => s.id !== sectionId);
      tasks.filter((t) => t.sectionId === sectionId).forEach((t) => (t.sectionId = null)); notify();
    },
    move(taskId: string, listId: string, parentId: string | null, sectionId: string | null, position: number): Task {
      const t = find(taskId);
      if (sectionId && (parentId || sections.find((s) => s.id === sectionId)?.listId !== listId)) throw { code: "invalid", message: "" };
      if (parentId && (find(parentId).parentId || tasks.some((k) => k.parentId === t.id))) throw { code: "too_deep", message: "" };
      t.listId = listId; t.parentId = parentId; t.sectionId = sectionId; t.position = position;
      tasks.filter((k) => k.parentId === t.id).forEach((k) => (k.listId = listId));
      notify(); return t;
    },
    remove(taskId: string): void { tasks = tasks.filter((t) => t.id !== taskId && t.parentId !== taskId); notify(); },
  };
}

export const fakeTasks = createFakeTasks();
