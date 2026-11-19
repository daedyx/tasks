import { isTauri } from "../tauriEnv";

export type CloseEvent = { preventDefault: () => void };

export type TraySetup = {
  tooltip: string;
  openLabel: string;
  quitLabel: string;
  onOpen: () => void;
  onQuit: () => void;
};

/** Alles, was für den Hintergrundlauf aus dem Fenster-System gebraucht wird (austauschbar für Tests). */
export type BackgroundApi = {
  /** Legt das Symbol im Infobereich an; die Rückgabe entfernt es wieder. */
  setupTray: (s: TraySetup) => Promise<() => void>;
  onCloseRequested: (handler: (e: CloseEvent) => void) => Promise<() => void>;
  hideWindow: () => Promise<void>;
  showWindow: () => Promise<void>;
  quit: () => Promise<void>;
  autostart: { isEnabled: () => Promise<boolean>; set: (on: boolean) => Promise<void> };
};

async function win() {
  return (await import("@tauri-apps/api/window")).getCurrentWindow();
}

export const tauriBackgroundApi: BackgroundApi = {
  async setupTray(s) {
    const { TrayIcon } = await import("@tauri-apps/api/tray");
    const { Menu } = await import("@tauri-apps/api/menu");
    const { defaultWindowIcon } = await import("@tauri-apps/api/app");
    const menu = await Menu.new({
      items: [
        { id: "open", text: s.openLabel, action: s.onOpen },
        { id: "quit", text: s.quitLabel, action: s.onQuit },
      ],
    });
    const tray = await TrayIcon.new({
      icon: (await defaultWindowIcon()) ?? undefined,
      tooltip: s.tooltip,
      menu,
      showMenuOnLeftClick: false,
      action: (e) => {
        if (e.type === "Click" && e.button === "Left" && e.buttonState === "Up") s.onOpen();
      },
    });
    return () => void tray.close();
  },
  async onCloseRequested(handler) {
    return (await win()).onCloseRequested((e) => handler(e));
  },
  async hideWindow() {
    await (await win()).hide();
  },
  async showWindow() {
    const w = await win();
    await w.show();
    await w.unminimize();
    await w.setFocus();
  },
  async quit() {
    const { exit } = await import("@tauri-apps/plugin-process");
    await exit(0);
  },
  autostart: {
    async isEnabled() {
      return (await import("@tauri-apps/plugin-autostart")).isEnabled();
    },
    async set(on) {
      const m = await import("@tauri-apps/plugin-autostart");
      await (on ? m.enable() : m.disable());
    },
  },
};

/** Die echte Schnittstelle im Tauri-Fenster, sonst `undefined` (Browser-Vorschau, Tests). */
export function defaultBackgroundApi(): BackgroundApi | undefined {
  return isTauri() ? tauriBackgroundApi : undefined;
}
