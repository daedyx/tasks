import { useEffect } from "react";

type Handler = (query: string) => void;

let pending: string | null = null;
const handlers = new Set<Handler>();

/** Bittet den Aufgabenbereich, Suchtreffer zu zeigen (auch wenn er noch nicht geöffnet ist). */
export function requestTaskSearch(query: string): void {
  pending = query;
  handlers.forEach((h) => h(query));
}

/** Im Aufgabenbereich: Suchwünsche entgegennehmen, auch einen, der vor dem Öffnen gestellt wurde. */
export function useTaskSearchRequest(onSearch: Handler): void {
  useEffect(() => {
    handlers.add(onSearch);
    if (pending !== null) {
      const q = pending;
      pending = null;
      onSearch(q);
    }
    return () => {
      handlers.delete(onSearch);
    };
  }, [onSearch]);
}
