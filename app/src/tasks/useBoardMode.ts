import { useCallback, useEffect, useState } from "react";

export type BoardMode = "list" | "board";
const key = (listId: string) => `daedyx.tasks.view.${listId}`;

function read(listId: string | null): BoardMode {
  if (!listId) return "list";
  try {
    return localStorage.getItem(key(listId)) === "board" ? "board" : "list";
  } catch {
    return "list";
  }
}

/** Liste oder Board, gemerkt je Liste (nur in diesem Gerät, `localStorage`). */
export function useBoardMode(listId: string | null): [BoardMode, (m: BoardMode) => void] {
  const [mode, setMode] = useState<BoardMode>(() => read(listId));
  useEffect(() => setMode(read(listId)), [listId]);
  const set = useCallback(
    (m: BoardMode) => {
      setMode(m);
      if (!listId) return;
      try {
        localStorage.setItem(key(listId), m);
      } catch {
        /* ohne Speicher gilt die Wahl nur für diese Sitzung */
      }
    },
    [listId],
  );
  return [mode, set];
}
