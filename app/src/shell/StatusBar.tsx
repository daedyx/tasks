import { t } from "../i18n";
import { useStartupCheck } from "../updates/useStartupCheck";

type Props = {
  /** Öffnet die Einstellungen (Hinweis „Update verfügbar“) */
  onUpdates: () => void;
};

export function StatusBar({ onUpdates }: Props) {
  const update = useStartupCheck();
  return (
    <footer className="statusbar">
      <div>
        {update && (
          <button type="button" className="statusbar__link" onClick={onUpdates}>
            {t("updates.status_available", { version: update.version })}
          </button>
        )}
      </div>
    </footer>
  );
}
