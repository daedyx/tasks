import { useEffect } from "react";
import { t } from "../i18n";
import { defaultBackgroundApi, type BackgroundApi } from "./backgroundApi";
import { readKeepRunning } from "./backgroundPrefs";

/**
 * Hält die App im Infobereich am Leben: Symbol mit Menü (Öffnen, Beenden) und „Fenster schließen“ versteckt nur,
 * damit Erinnerungen auch bei geschlossenem Fenster kommen. Der Wert der Einstellung wird erst beim Schließen gelesen.
 */
export function useBackgroundRun(api: BackgroundApi | undefined = defaultBackgroundApi()): void {
  useEffect(() => {
    if (!api) return;
    let on = true;
    const offs: Array<() => void> = [];
    const keep = (off: () => void) => (on ? offs.push(off) : off());

    void api
      .setupTray({
        tooltip: t("background.tray.tooltip"),
        openLabel: t("background.tray.open"),
        quitLabel: t("background.tray.quit"),
        onOpen: () => void api.showWindow(),
        onQuit: () => void api.quit(),
      })
      .then(keep)
      .catch(() => undefined);
    void api
      .onCloseRequested((e) => {
        if (!readKeepRunning()) return;
        e.preventDefault();
        void api.hideWindow();
      })
      .then(keep)
      .catch(() => undefined);

    return () => {
      on = false;
      offs.forEach((off) => off());
    };
  }, [api]);
}
