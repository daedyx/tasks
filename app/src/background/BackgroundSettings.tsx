import { useEffect, useState } from "react";
import { t } from "../i18n";
import { defaultBackgroundApi, type BackgroundApi } from "./backgroundApi";
import { readKeepRunning, writeKeepRunning } from "./backgroundPrefs";

type Props = { api?: BackgroundApi };

/** Einstellungen zum Hintergrundlauf: im Infobereich weiterlaufen und mit Windows starten. */
export function BackgroundSettings({ api = defaultBackgroundApi() }: Props) {
  const [keep, setKeep] = useState(readKeepRunning);
  const [auto, setAuto] = useState(false);
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let on = true;
    if (!api) return;
    api.autostart
      .isEnabled()
      .then((v) => on && (setAuto(v), setReady(true)))
      .catch(() => undefined);
    return () => {
      on = false;
    };
  }, [api]);

  const toggleAuto = (next: boolean) => {
    setError(null);
    api?.autostart
      .set(next)
      .then(() => setAuto(next))
      .catch((e: unknown) =>
        setError(t("background.autostart.error", { message: e instanceof Error ? e.message : String(e) })),
      );
  };

  return (
    <>
      <h3 className="settings__heading">{t("background.title")}</h3>
      <div className="settings__row">
        <div>
          <label className="settings__label" htmlFor="bg-keep">
            {t("background.keep")}
          </label>
          <p className="settings__hint">{t("background.keep_hint")}</p>
        </div>
        <input id="bg-keep" type="checkbox" checked={keep} onChange={(e) => (setKeep(e.target.checked), writeKeepRunning(e.target.checked))} />
      </div>
      <div className="settings__row">
        <div>
          <label className="settings__label" htmlFor="bg-auto">
            {t("background.autostart")}
          </label>
          <p className="settings__hint">{t("background.autostart_hint")}</p>
        </div>
        <input id="bg-auto" type="checkbox" checked={auto} disabled={!ready} onChange={(e) => toggleAuto(e.target.checked)} />
      </div>
      {error && (
        <p className="settings__hint" role="alert">
          {error}
        </p>
      )}
    </>
  );
}
