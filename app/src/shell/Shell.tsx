import type { ReactNode } from "react";
import { t } from "../i18n";
import type { ThemeMode } from "../theme/useTheme";
import { Icon, type IconName } from "../ui/Icon";
import { StatusBar } from "./StatusBar";
import "./shell.css";

const THEME_VIEW: Record<ThemeMode, { icon: IconName; labelKey: string }> = {
  system: { icon: "monitor", labelKey: "theme.system" },
  light: { icon: "sun", labelKey: "theme.light" },
  dark: { icon: "moon", labelKey: "theme.dark" },
};

type Props = {
  settingsOpen: boolean;
  onToggleSettings: () => void;
  themeMode: ThemeMode;
  onThemeCycle: () => void;
  onUpdates: () => void;
  children: ReactNode;
};

/** Rahmen der App: Kopfzeile mit den zwei Werkzeugen, Inhalt, Statusleiste. Der Inhalt ist der Aufgabenbereich. */
export function Shell({ settingsOpen, onToggleSettings, themeMode, onThemeCycle, onUpdates, children }: Props) {
  const theme = THEME_VIEW[themeMode];
  return (
    <div className="shell">
      <header className="shell__top">
        <h1 className="shell__title">{t("app.name")}</h1>
        <div className="shell__tools">
          <button className="shell__tool" title={t("theme.switch")} onClick={onThemeCycle}>
            <Icon name={theme.icon} size={18} />
            <span className="visually-hidden">{t(theme.labelKey)}</span>
          </button>
          <button
            className="shell__tool"
            title={t("settings.title")}
            aria-pressed={settingsOpen}
            onClick={onToggleSettings}
          >
            <Icon name="sliders" size={18} />
            <span className="visually-hidden">{t("settings.open")}</span>
          </button>
        </div>
      </header>
      <main className="shell__content">{children}</main>
      <StatusBar onUpdates={onUpdates} />
    </div>
  );
}
