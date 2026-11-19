// Dünne Hülle um die Tauri-Plugins für Updates und Neustart; in Tests wird sie durch eine Attrappe ersetzt.
import { isTauri } from "../tauriEnv";

/** Fehlercode, den die Oberfläche über i18n in Text umsetzt */
export const NO_UPDATE = "no_update";

export type UpdateInfo = { version: string; notes: string };

export interface UpdaterApi {
  currentVersion(): Promise<string>;
  /** `null`, wenn die installierte Version die neueste ist */
  check(): Promise<UpdateInfo | null>;
  /** Lädt und installiert die zuletzt von `check` gefundene Version; meldet Fortschritt in Bytes */
  install(onProgress: (done: number, total: number) => void): Promise<void>;
  restart(): Promise<void>;
}

type FoundUpdate = import("@tauri-apps/plugin-updater").Update;

let found: FoundUpdate | null = null;

export const tauriUpdater: UpdaterApi = {
  async currentVersion() {
    if (!isTauri()) return "";
    return (await import("@tauri-apps/api/app")).getVersion();
  },
  async check() {
    if (!isTauri()) return null;
    const update = await (await import("@tauri-apps/plugin-updater")).check();
    found = update;
    return update ? { version: update.version, notes: update.body ?? "" } : null;
  },
  async install(onProgress) {
    if (!found) throw new Error(NO_UPDATE);
    let done = 0;
    let total = 0;
    await found.downloadAndInstall((event) => {
      if (event.event === "Started") total = event.data.contentLength ?? 0;
      if (event.event === "Progress") {
        done += event.data.chunkLength;
        onProgress(done, total);
      }
    });
  },
  async restart() {
    if (!isTauri()) return;
    await (await import("@tauri-apps/plugin-process")).relaunch();
  },
};
