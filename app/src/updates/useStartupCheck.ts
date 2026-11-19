import { useEffect, useState } from "react";
import { useStoredFlag } from "../ui/useStoredFlag";
import { AUTO_KEY, readLastCheck, shouldAutoCheck, writeLastCheck } from "./autoCheck";
import { tauriUpdater, type UpdateInfo, type UpdaterApi } from "./updater";

/** Sucht beim Start (höchstens einmal pro Tag) leise nach einem Update; Fehler bleiben stumm. Liefert die gefundene Version. */
export function useStartupCheck(api: UpdaterApi = tauriUpdater): UpdateInfo | null {
  const [auto] = useStoredFlag(AUTO_KEY, true);
  const [found, setFound] = useState<UpdateInfo | null>(null);

  useEffect(() => {
    const now = Date.now();
    if (!shouldAutoCheck(now, readLastCheck(), auto)) return;
    let gone = false;
    api.check().then(
      (info) => {
        writeLastCheck(now);
        if (!gone) setFound(info);
      },
      () => undefined,
    );
    return () => {
      gone = true;
    };
  }, [api, auto]);

  return found;
}
