import { useCallback, useState, type CSSProperties } from "react";
import { SettingsPage } from "./settings/SettingsPage";
import { Shell } from "./shell/Shell";
import { TasksArea } from "./tasks/TasksArea";
import { useDarkStyle } from "./theme/useDarkStyle";
import { useTheme } from "./theme/useTheme";

/** Akzentfarbe von Tasks (Token-Namen aus `tokens.css`); frei wählbare App-Farben kommen mit Paket K5. */
const ACCENT = {
  "--accent": "var(--c-green)",
  "--accent-contrast": "var(--c-green-on)",
  "--bg-selected": "var(--c-green-soft)",
  "--focus": "var(--c-green)",
} as CSSProperties;

export default function App() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const theme = useTheme();
  const dark = useDarkStyle();
  const toggleSettings = useCallback(() => setSettingsOpen((open) => !open), []);
  const openSettings = useCallback(() => setSettingsOpen(true), []);

  return (
    <div className="app-root" style={ACCENT}>
      <Shell
        settingsOpen={settingsOpen}
        onToggleSettings={toggleSettings}
        themeMode={theme.mode}
        onThemeCycle={theme.cycle}
        onUpdates={openSettings}
      >
        {settingsOpen ? (
          <SettingsPage darkStyle={dark.style} onDarkStyle={dark.update} onReset={dark.reset} />
        ) : (
          <TasksArea />
        )}
      </Shell>
    </div>
  );
}
