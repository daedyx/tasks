import { useCallback, useEffect, useRef, useState } from "react";
import { t } from "../i18n";
import { NO_UPDATE, tauriUpdater, type UpdateInfo, type UpdaterApi } from "./updater";

export type UpdateState = "idle" | "checking" | "uptodate" | "available" | "downloading" | "ready" | "error" | "unavailable";

function message(e: unknown): string {
  if (e instanceof Error && e.message === NO_UPDATE) return t("updates.none_found");
  return e instanceof Error ? e.message : String(e);
}

/** Ohne eingetragenen Update-Schlüssel hängt die App das Plugin gar nicht ein; Tauri meldet dann „plugin updater not found“. */
function notSetUp(e: unknown): boolean {
  return /updater not found/i.test(e instanceof Error ? e.message : String(e));
}

/** Zustand der Update-Suche und -Installation. Fehler werden als Zustand gemeldet, nie geworfen. */
export function useUpdater(api: UpdaterApi = tauriUpdater) {
  const [state, setState] = useState<UpdateState>("idle");
  const [info, setInfo] = useState<UpdateInfo | null>(null);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [version, setVersion] = useState("");
  const current = useRef(state);
  current.current = state;

  useEffect(() => {
    let gone = false;
    api.currentVersion().then(
      (v) => !gone && setVersion(v),
      () => undefined,
    );
    return () => {
      gone = true;
    };
  }, [api]);

  const check = useCallback(async () => {
    setState("checking");
    setError(null);
    try {
      const found = await api.check();
      setInfo(found);
      setState(found ? "available" : "uptodate");
    } catch (e) {
      if (notSetUp(e)) {
        setState("unavailable");
        return;
      }
      setError(message(e));
      setState("error");
    }
  }, [api]);

  /** `confirm` fragt vorher nach ungespeicherten Entwürfen; `false` hält das Installieren an. */
  const install = useCallback(
    async (confirm?: () => Promise<boolean>) => {
      if (confirm && !(await confirm())) return;
      setState("downloading");
      setProgress(0);
      try {
        await api.install((done, total) => setProgress(total > 0 ? done / total : 0));
        setProgress(1);
        setState("ready");
      } catch (e) {
        setError(message(e));
        setState("error");
      }
    },
    [api],
  );

  const restart = useCallback(async () => {
    if (current.current !== "ready") return;
    await api.restart();
  }, [api]);

  return { state, info, progress, error, version, check, install, restart };
}
