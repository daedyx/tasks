import { BackgroundSettings } from "../background/BackgroundSettings";
import { t } from "../i18n";
import type { DarkStyle } from "../theme/darkStyle";
import { DarkStyleSection } from "./DarkStyleSection";
import { UpdatesSettings } from "./UpdatesSettings";
import "./settings.css";

type Props = { darkStyle: DarkStyle; onDarkStyle: (patch: Partial<DarkStyle>) => void; onReset: () => void };

/** Einstellungen von Tasks: Darstellung (Dunkelmodus) und Updates. Weitere Kategorien kommen mit den Paketen K4/K5. */
export function SettingsPage({ darkStyle, onDarkStyle, onReset }: Props) {
  return (
    <div className="settings">
      <div className="settings__content">
        <h2 className="settings__title">{t("settings.title")}</h2>
        <DarkStyleSection style={darkStyle} onChange={onDarkStyle} />
        <button type="button" className="cbtn" onClick={onReset}>
          {t("settings.reset")}
        </button>
        <BackgroundSettings />
        <UpdatesSettings />
      </div>
    </div>
  );
}
