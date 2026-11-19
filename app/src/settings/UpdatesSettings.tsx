import { useEffect } from "react";
import { t } from "../i18n";
import { AUTO_KEY } from "../updates/autoCheck";
import type { UpdaterApi } from "../updates/updater";
import { useUpdater } from "../updates/useUpdater";
import { useStoredFlag } from "../ui/useStoredFlag";
import "../ui/buttons.css";
import "./updates.css";

type Props = { api?: UpdaterApi };

export function UpdatesSettings({ api }: Props) {
  const u = useUpdater(api);
  const [auto, setAuto] = useStoredFlag(AUTO_KEY, true);
  const { check } = u;

  // Beim Öffnen der Seite einmal suchen
  useEffect(() => {
    void check();
  }, [check]);

  const busy = u.state === "checking" || u.state === "downloading";

  return (
    <>
      <h3 className="settings__heading">{t("updates.title")}</h3>
      <p className="settings__hint">{t("updates.version", { version: u.version })}</p>

      <div className="updates__status" role="status">
        {u.state === "checking" && t("updates.checking")}
        {u.state === "uptodate" && t("updates.uptodate")}
        {u.state === "available" && u.info && t("updates.available", { version: u.info.version })}
        {u.state === "downloading" && t("updates.installing")}
        {u.state === "ready" && t("updates.ready")}
        {u.state === "error" && t("updates.error", { text: u.error ?? "" })}
      </div>

      {u.state === "available" && u.info && (
        <>
          {u.info.notes !== "" && (
            <>
              <h4 className="settings__subheading">{t("updates.notes")}</h4>
              <p className="updates__notes">{u.info.notes}</p>
            </>
          )}
          <p className="settings__hint">{t("updates.install_hint")}</p>
        </>
      )}

      {u.state === "downloading" && (
        <div
          className="updates__bar"
          role="progressbar"
          aria-label={t("updates.progress")}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(u.progress * 100)}
        >
          <div className="updates__barfill" style={{ transform: `scaleX(${u.progress})` }} />
        </div>
      )}

      <div className="updates__actions">
        <button type="button" className="cbtn" disabled={busy} onClick={() => void u.check()}>
          {t("updates.check")}
        </button>
        {u.state === "available" && (
          <button
            type="button"
            className="cbtn cbtn--primary"
            onClick={() => void u.install()}
          >
            {t("updates.install")}
          </button>
        )}
        {u.state === "ready" && (
          <button type="button" className="cbtn cbtn--primary" onClick={() => void u.restart()}>
            {t("updates.restart")}
          </button>
        )}
      </div>

      <div className="settings__row">
        <div>
          <label className="settings__label" htmlFor="updates-auto">
            {t("updates.auto")}
          </label>
          <p className="settings__hint">{t("updates.auto_hint")}</p>
        </div>
        <input id="updates-auto" type="checkbox" checked={auto} onChange={(e) => setAuto(e.target.checked)} />
      </div>
    </>
  );
}
