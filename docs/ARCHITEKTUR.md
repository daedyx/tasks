# Architektur Daedyx Tasks

Verbindliche Grundlage ist das Konzept `daedyx-desk/docs/superpowers/plans/2026-11-18-daedyx-tasks-konzept.md` (Abschnitte 2, 5, 7, 8, 9) und die Suite-Entscheidungen in `daedyx-desk/docs/SUITE-DAEDYX.md`. Hier steht nur, was dieses Repository festlegt.

## Aufbau
- `app/` Tauri 2: React + TypeScript (`app/src`), Rust (`app/src-tauri`).
- Rechenkern: Crate `daedyx-tasks-core` aus dem privaten Repository `daedyx/core`, **Git-Abhängigkeit mit Tag** (`tasks-core-v0.1.0`, Commit `9d677ee`). Die App enthält kein Aufgaben-Schema und keine Aufgaben-Logik: `commands_tasks.rs` ist eine dünne Hülle (Tauri-Befehle, Ereignis `tasks://changed`), `tasks/background.rs` der Erinnerungstakt (30 s, Ereignis `tasks://reminder`).
- Der Kern öffnet keine Datenbank; die App öffnet sie (SQLCipher) und ruft beim Start `daedyx_tasks_core::migrate`. Eigene Schema-Zählung im Kern (`task_meta.schema_version`), keine `user_version`-Migrationen in dieser App. Eine Datenbank mit höherem Stand wird abgelehnt (`TooNew`), nicht verändert.
- Oberfläche: nur der Aufgabenbereich (`src/tasks`, aus Desk kopiert und ab jetzt eigenständig, Konzept 2.5), eine schlanke Hülle (`src/shell`: Kopfzeile mit Designwechsel und Einstellungen, Statusleiste mit Update-Hinweis) und Einstellungen (`src/settings`: Dunkelmodus, Über und Updates). Kein Mail, kein Kalender, keine Konten, keine Schnellsuche-Palette.

## Identität
| | Installiert | Entwicklung (`npm run tauri:dev`, Merkmal `dev-bridge`) |
|---|---|---|
| Kennung | `de.daedyx.tasks` | `de.daedyx.tasks.dev` |
| Datenordner | `%APPDATA%\de.daedyx.tasks` | `%APPDATA%\de.daedyx.tasks.dev` |
| Datenbank | `daedyx-tasks.db` | dieselbe Datei im Dev-Ordner |
| Tresor-Dienst | `daedyx-tasks` (Eintrag `db-key`) | `daedyx-tasks-dev` |
| Vite-Port | — | 1430 (Desk nutzt 1420, beide können nebeneinander laufen) |

Ein Wächtertest (`config_tests.rs`) hält Kennung, Dev-Kennung, Tresor-Dienst, Installer-Einstellungen, `app.json`-Hook, Updater-Kanal und die Git-Einbindung des Kerns fest.

## Installer und Katalog
NSIS pro Benutzer (`installMode: currentUser`, Deutsch). Der Hook `windows/installer-hooks.nsh` schreibt nach jeder Installation `app.json` (`version`, `exe`) in den Installationsordner (Launcher-Vertrag) und löscht sie beim Deinstallieren. Der Registry-Eintrag für das Link-Schema `daedyx-tasks` gehört zu Paket K6 und fehlt noch. Katalog-Eintrag: App-Kennung `tasks`, `registryKey` „Daedyx Tasks“ (`scripts/lib/release-cli.mjs`). Veröffentlichung: `docs/RELEASE.md`.

## Updater
`tauri.conf.json` hat den Kanal `https://raw.githubusercontent.com/daedyx/releases/main/tasks/latest.json` und einen **leeren** öffentlichen Schlüssel. Solange er leer ist, ist `createUpdaterArtifacts` aus und der Updater wird in `lib.rs` nicht eingehängt (eine ungültige Updater-Konfiguration würde den Start verhindern). Mit Björns Schlüssel: beides zusammen einschalten (Anleitung `docs/RELEASE.md`).

## Bewusst noch nicht da
Alles aus K4 bis K8 des Konzepts: mehrstufige Unteraufgaben, Monatstag-Anker, mehrere Erinnerungen, Erinnerung bei geschlossenem Fenster (Infobereich, Autostart), freie Farben und App-Farben, Link-Schema, Single-Instance, Pipe, Umzug der Desk-Listen, Rückverweise. Der Kern ist auf Stand 2 (Abschnitte); Stand 3 (Farben, Links, `home`) ist Paket K2.
