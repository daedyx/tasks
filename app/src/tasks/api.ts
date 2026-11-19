// Einzige Stelle der Oberfläche mit invoke/listen für die Aufgaben. Ohne Tauri (Vite-Vorschau, Tests) antwortet
// im Entwicklungsbetrieb ein Fake mit erfundenen Daten; der Produktionsbau enthält ihn nicht.
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { toTaskError } from "./errors";
import type { FakeTasks } from "./fakeTasks";
import type { Section, Task, TaskList, TaskPatch, TaskReminder, Unlisten, View, ViewCount } from "./types";

const isTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
const useFake = (): boolean => import.meta.env.DEV && !isTauri();

async function fake(): Promise<FakeTasks> {
  if (!import.meta.env.DEV) throw toTaskError({ code: "internal", message: "kein Kern" });
  return (await import("./fakeTasks")).fakeTasks;
}

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (e) {
    throw toTaskError(e);
  }
}

const zone = (): string => Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";

async function run<T>(real: () => Promise<T>, fk: (f: FakeTasks) => T | Promise<T>): Promise<T> {
  if (!useFake()) return real();
  try {
    return await fk(await fake());
  } catch (e) {
    throw toTaskError(e);
  }
}

export const listTaskLists = (): Promise<TaskList[]> => run(() => call("task_list_list"), (f) => f.lists());
export const createTaskList = (name: string, color = ""): Promise<TaskList> =>
  run(() => call("task_list_create", { name, color }), (f) => f.createList(name, color));
export const updateTaskList = (id: string, change: { name?: string; color?: string }): Promise<void> =>
  run(() => call("task_list_update", { id, ...change }), (f) => f.updateList(id, change.name, change.color));
export const reorderTaskLists = (ids: string[]): Promise<void> => run(() => call("task_list_reorder", { ids }), (f) => f.reorderLists(ids));
export const deleteTaskList = (id: string): Promise<void> => run(() => call("task_list_delete", { id }), (f) => f.deleteList(id));

export const queryTasks = (view: View, listId: string | null = null): Promise<Task[]> =>
  run(() => call("task_query", { view, listId, tz: zone() }), (f) => f.query(view, listId));
export const taskCounts = (): Promise<ViewCount[]> => run(() => call("task_counts", { tz: zone() }), (f) => f.counts());
export const searchTasks = (query: string): Promise<Task[]> => run(() => call("task_search", { query }), (f) => f.search(query));

export const createTask = (listId: string, fields: TaskPatch, parentId: string | null = null): Promise<Task> =>
  run(() => call("task_create", { listId, parentId, fields }), (f) => f.create(listId, parentId, fields));
export const updateTask = (id: string, patch: TaskPatch): Promise<Task> =>
  run(() => call("task_update", { id, patch }), (f) => f.update(id, patch));
export const toggleTask = (id: string): Promise<Task> => run(() => call("task_toggle_done", { id, tz: zone() }), (f) => f.toggle(id));
/** `sectionId`: gewünschter Abschnitt (null = „Ohne Abschnitt“); bei Listenwechsel muss er zur Zielliste gehören. */
export const moveTask = (id: string, listId: string, parentId: string | null, sectionId: string | null, position: number): Promise<Task> =>
  run(() => call("task_move", { id, listId, parentId, sectionId, position }), (f) => f.move(id, listId, parentId, sectionId, position));
/** Erinnerungen der Aufgabe: Minuten vor der Fälligkeit, aufsteigend. */
export const getTaskAlarms = (id: string): Promise<number[]> => run(() => call("task_alarms_get", { id }), (f) => f.alarms(id));
export const setTaskAlarms = (id: string, minutes: number[]): Promise<number[]> =>
  run(() => call("task_alarms_set", { id, minutes }), (f) => f.setAlarms(id, minutes));
export const deleteTask = (id: string): Promise<void> => run(() => call("task_delete", { id }), (f) => f.remove(id));

export const listSections = (listId: string): Promise<Section[]> =>
  run(() => call("task_section_list", { listId }), (f) => f.sections(listId));
export const createSection = (listId: string, name: string): Promise<Section> =>
  run(() => call("task_section_create", { listId, name }), (f) => f.createSection(listId, name));
export const renameSection = (id: string, name: string): Promise<void> =>
  run(() => call("task_section_rename", { id, name }), (f) => f.renameSection(id, name));
export const reorderSections = (listId: string, ids: string[]): Promise<void> =>
  run(() => call("task_section_reorder", { listId, ids }), (f) => f.reorderSections(listId, ids));
export const deleteSection = (id: string): Promise<void> =>
  run(() => call("task_section_delete", { id }), (f) => f.deleteSection(id));

export async function onTasksChanged(cb: () => void): Promise<Unlisten> {
  if (useFake()) return (await fake()).subscribe(cb);
  return listen("tasks://changed", () => cb());
}

export async function onTaskReminder(cb: (r: TaskReminder) => void): Promise<Unlisten> {
  if (useFake()) return () => undefined;
  return listen<TaskReminder>("tasks://reminder", (e) => cb(e.payload));
}
