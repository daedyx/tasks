import { useCallback, useEffect, useRef, useState } from "react";
import { getTaskAlarms, listSections, listTaskLists, onTasksChanged, queryTasks, searchTasks, taskCounts } from "./api";
import { toTaskError, type TaskError } from "./errors";
import type { Section, Task, TaskList, View, ViewCount } from "./types";

type Live<T> = { data: T; loaded: boolean; error: TaskError | null; reload: () => void };

/** Lädt `load` neu, wenn sich `key` ändert oder der Kern Änderungen meldet; behält die alten Daten, bis neue da sind. */
function useLive<T>(load: () => Promise<T>, initial: T, key: string): Live<T> {
  const [data, setData] = useState<T>(initial);
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState<TaskError | null>(null);
  const [tick, setTick] = useState(0);
  const loadRef = useRef(load);
  loadRef.current = load;
  const reload = useCallback(() => setTick((n) => n + 1), []);

  useEffect(() => {
    let current = true;
    loadRef
      .current()
      .then((d) => current && (setData(d), setError(null), setLoaded(true)))
      .catch((e) => current && (setError(toTaskError(e)), setLoaded(true)));
    return () => {
      current = false;
    };
  }, [key, tick]);

  useEffect(() => {
    let on = true;
    let off: (() => void) | undefined;
    void onTasksChanged(reload).then((u) => (on ? (off = u) : u()));
    return () => {
      on = false;
      off?.();
    };
  }, [reload]);

  return { data, loaded, error, reload };
}

const NO_LISTS: TaskList[] = [];
const NO_COUNTS: ViewCount[] = [];
const NO_TASKS: Task[] = [];
const NO_SECTIONS: Section[] = [];
const NO_ALARMS: number[] = [];

export const useTaskLists = () => useLive(listTaskLists, NO_LISTS, "lists");
export const useViewCounts = () => useLive(taskCounts, NO_COUNTS, "counts");
export const useTaskView = (view: View, listId: string | null) =>
  useLive(() => (view === "list" && !listId ? Promise.resolve(NO_TASKS) : queryTasks(view, listId)), NO_TASKS, `${view}/${listId ?? ""}`);
/** Suchtreffer; ohne Suchtext leer. */
export const useTaskSearch = (query: string | null) =>
  useLive(() => (query ? searchTasks(query) : Promise.resolve(NO_TASKS)), NO_TASKS, `search/${query ?? ""}`);
/** Abschnitte einer Liste (leer ohne Liste); kann kurz noch die der vorigen Liste zeigen, daher nach `listId` filtern. */
export const useSections = (listId: string | null) =>
  useLive(() => (listId ? listSections(listId) : Promise.resolve(NO_SECTIONS)), NO_SECTIONS, `sections/${listId ?? ""}`);
/** Erinnerungen einer Aufgabe (leer ohne Aufgabe). */
export const useTaskAlarms = (taskId: string | null) =>
  useLive(() => (taskId ? getTaskAlarms(taskId) : Promise.resolve(NO_ALARMS)), NO_ALARMS, `alarms/${taskId ?? ""}`);
