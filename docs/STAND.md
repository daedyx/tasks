# Stand — Daedyx Tasks

Letzte Änderung: 2026-11-19, Paket K4 Teil 1 und 2 „Tasks-Funktionen“ im Zweig `k4-funktionen` (nicht in `main`, Merge entscheidet Björn nach der Abnahme `docs/abnahme-k4.md`). Version 0.1.0, nichts veröffentlicht.

## Was gebaut ist
- Tauri-2-Projekt (React/TS + Rust), aus Daedyx Desk `main` (0.6.0, Stand nach Merge von K1) abgeleitet. Nur der Aufgabenbereich (Listen, Abschnitte, Board, Unteraufgaben eine Ebene, Wiederholung, Erinnerung, Smart-Listen) als Hauptfenster, dazu Einstellungen (Dunkelmodus, Über und Updates). Kein Mail, kein Kalender, keine Konten, keine Schnellsuche-Palette.
- Eigene Identität: `de.daedyx.tasks` (Dev `.dev`), Datenbank `daedyx-tasks.db` (SQLCipher), Tresor-Dienst `daedyx-tasks` (Dev `daedyx-tasks-dev`), Vite-Port 1430. Einzelheiten: `docs/ARCHITEKTUR.md`.
- Kern `daedyx-tasks-core` aus `daedyx/core` per Git-Abhängigkeit, Tag `tasks-core-v0.1.0` (= `9d677ee`, Schema-Stand 2).
- NSIS-Installer pro Benutzer mit `app.json`-Hook; Katalog-Eintrag (`tasks`) und Release-Workflow als Dateien vorbereitet (`scripts/`, `.github/workflows/release.yml`, `release/catalog.pub`).

## K4 (Zweig k4-funktionen), Stand 2026-11-19
**Gebaut ohne Kernänderung:** Board mit beliebig vielen Abschnitten (Test mit 14); Hintergrundlauf: Symbol im Infobereich (Öffnen, Beenden), „Fenster schließen“ versteckt nur (Schalter, Standard an), Autostart (Argument `--hidden`, Schalter in den Einstellungen), Einzelinstanz (zweiter Start holt das Fenster nach vorn). Code: `app/src/background/`, `app/src-tauri/src/background.rs`, `lib.rs`. Belegt: `npm test` 168 grün, `tsc` sauber, `cargo test` 17 grün (1 ignoriert), Wächtertest für Berechtigungen und Plugins. **Nicht von Hand im echten Fenster geprüft** (Symbol, Benachrichtigung bei geschlossenem Fenster, Autostart): das ist `docs/abnahme-k4.md`.
**Teil 2 (Kern Stand 3 per `rev = "86ea2fc"`, Zweig `k2-schema-3` in daedyx/core, noch ohne Tag, danach auf den Tag umstellen):**
- Mehrere Erinnerungen je Aufgabe (bis zehn): Befehle `task_alarms_get/set`, Oberfläche `AlarmFields`, Hook `useTaskAlarms`. Fälligkeit entfernen löscht sie.
- Fester Monatstag (`monthDay`) für monatlich und jährlich im Detailbereich; Fake-Kern (`repeatNext.ts`) ohne Klemmen auf 28.
- Unteraufgaben bis Ebene 6 (`TASKS_MAX_DEPTH` in `limits.ts` und `limits.rs`, der Kern bekommt `max_depth`): Einrückung nach Ebene, Zähler über alle Nachfahren, Sprung zur Oberaufgabe, „Öffnen“ in der Unteraufgabenliste.
- Belegt: `npm test` 193 grün, `tsc` und `npm run build` sauber, `cargo test` 19 grün (1 ignoriert; u. a. Ebene 6 geht, Ebene 7 ergibt `TooDeep`; Erinnerungen sortiert und ohne Doppelte). **Nicht von Hand im echten Fenster geprüft:** `docs/abnahme-k4.md` F–H.
- **Offen/Risiko:** (1) Der Kern hat noch keinen Tag; `Cargo.toml` zeigt auf den Commit. (2) Eine bestehende Datenbank wird beim ersten Start von Stand 2 auf 3 gehoben; Tasks legt davor noch keine Sicherung an (Handkopie in der Abnahmeliste). (3) Eine alte App (Stand 2) verwirft `monthDay`, wenn sie die Regel neu schreibt. (4) Test `api.contract.test.ts` liest Fehlercodes aus der erstbesten Kern-Kopie im Cargo-Cache.

## Belegt (2026-11-19, K3)
- Rust `cargo test`: 15 grün, 1 ignoriert (echter Tresor, nur von Hand). Darunter: Datenbank verschlüsselt, falscher oder ungültiger Schlüssel, erster Start legt Schema an, zweiter Start behält Daten, neuerer Kern-Stand wird abgelehnt; Wächtertests für Kennung, Dev-Trennung, Installer, Updater, Git-Einbindung.
- Oberfläche `npm test`: 156 grün (27 Dateien), `tsc` und `npm run build` sauber. App-Test: startet im Aufgabenbereich, Liste anlegen, Aufgabe schreiben und abhaken, Einstellungen, Designwechsel (alles gegen das Fake-Backend der Oberfläche).
- Skripte `npm run test:scripts`: 32 grün. Trockenprobe der Veröffentlichung `scripts/ci/test-publish.ps1` ohne Netz (Wegwerf-Schlüssel, Launcher-CLI liest `tasks` im Katalog): bestanden.
- Installer: `npm run tauri build` baut `Daedyx Tasks_0.1.0_x64-setup.exe` (4,13 MiB); Installationsprobe (`scripts/test-installer.ps1`: still installieren, `app.json` mit Version und Programm prüfen, deinstallieren) bestanden.
- Start der gebauten App (`daedyx-tasks.exe`, normale Kennung): Fenster-Prozess lebt, Datenordner mit `daedyx-tasks.db` (+WAL) und Tresor-Eintrag werden angelegt. Danach Prozess beendet und beides (Datenordner, Tresor-Eintrag) wieder gelöscht.

## Nicht geprüft
- Bedienung im echten Fenster (Liste anlegen, abhaken, Ziehen) und Aussehen hell/dunkel: nur in Tests gegen das Fake-Backend, nicht im Tauri-Fenster. Verbindung Oberfläche ↔ Rust über die Befehle ist durch den Schnittstellentest (`tasks/api.contract.test.ts`) belegt, nicht von Hand.
- Start über den Launcher aus einem lokalen Katalog (Konzept-Kriterium K3), Deinstallation über den Launcher, echter Workflow-Lauf in GitHub Actions, Updater (Schlüssel fehlt), Erinnerung als Windows-Benachrichtigung.
- Sicherung der Datenbank vor einem Kern-Upgrade (Desk hat `backup_before_migrate`, hier noch nicht; nötig spätestens mit Schema-Stand 3, K2).

## Offen für Björn (Details `docs/RELEASE.md`)
1. Update-Schlüssel für Tasks erzeugen, Passwort selbst tippen, privat an zwei Orten sichern, öffentlichen Teil einem Bau-Chat geben.
2. Sechs Secrets in `daedyx/tasks`: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`, `DAEDYX_CATALOG_KEY`, `DAEDYX_KEY_PASSWORD`, `RELEASES_TOKEN`, neu `CORE_READ_TOKEN` (Fine-grained, nur `daedyx/core`, Contents Read-only).
3. Erster Release-Lauf erst auf Björns Go.

## Hinweise
- Lizenz: keine gewählt, keine Datei angelegt. Icon: vorläufig das von Desk.
- Vor dem gemeinsamen Veröffentlichen mit Desk: `scripts/ci/publish.ps1` von Desk angleichen (überschreibt `.gitattributes` im Katalog, siehe `docs/RELEASE.md`).
- Die Oberfläche wurde mit `scripts/trim-i18n.mjs` einmalig von Desk-Texten befreit; das Skript ist nur Herkunftsnachweis.
- Nächste Pakete laut Konzept: K2 (Schema Stand 3, liegt im Kern), K4 und K5 (Funktionen, Farben), K6 (Link-Schema, Single-Instance).

## Befehle
- Rust: in PowerShell `$env:Path = "C:\Strawberry\perl\bin;C:\Strawberry\c\bin;" + $env:Path + ";$env:USERPROFILE\.cargo\bin"`, dann `cd app\src-tauri; cargo test` (erster Bau ca. 6 Minuten).
- Oberfläche: `cd app; npm test`; Dev-App mit Konnektor: `npm run tauri:dev`; Installer: `npm run tauri build`.

## Offene Punkte aus dem Review der Härtung (2026-11-21)
- **M3 – Tasks hat weder Pflicht-Sicherung noch Startfehler-Dialog (vor 0.1.1, vor der ersten Migration):** `db::open_and_migrate` migriert ohne Kopie davor, und ein Startfehler beendet die App ohne Meldung. Für 0.1.0 folgenlos (neue Datei, nichts zu sichern). Vor der ersten Folgeversion mit Kern-Migration den Sicherungs- und Startfehler-Code aus Desk (`db/backup.rs`, `startup.rs`) übernehmen.
- **N5 – Tag-Weg der Veröffentlichung:** Ein Tag auf einem alten main-Commit veröffentlicht diesen alten Stand ohne Bestätigung und ohne Trockenlauf. Entscheidung für Björn: Zweitweg behalten oder streichen.
