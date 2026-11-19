# Stand — Daedyx Tasks

Letzte Änderung: 2026-11-19, Paket K3 „Tasks-Gerüst“ (Bau-Chat). Version 0.1.0, nichts veröffentlicht.

## Was gebaut ist
- Tauri-2-Projekt (React/TS + Rust), aus Daedyx Desk `main` (0.6.0, Stand nach Merge von K1) abgeleitet. Nur der Aufgabenbereich (Listen, Abschnitte, Board, Unteraufgaben eine Ebene, Wiederholung, Erinnerung, Smart-Listen) als Hauptfenster, dazu Einstellungen (Dunkelmodus, Über und Updates). Kein Mail, kein Kalender, keine Konten, keine Schnellsuche-Palette.
- Eigene Identität: `de.daedyx.tasks` (Dev `.dev`), Datenbank `daedyx-tasks.db` (SQLCipher), Tresor-Dienst `daedyx-tasks` (Dev `daedyx-tasks-dev`), Vite-Port 1430. Einzelheiten: `docs/ARCHITEKTUR.md`.
- Kern `daedyx-tasks-core` aus `daedyx/core` per Git-Abhängigkeit, Tag `tasks-core-v0.1.0` (= `9d677ee`, Schema-Stand 2).
- NSIS-Installer pro Benutzer mit `app.json`-Hook; Katalog-Eintrag (`tasks`) und Release-Workflow als Dateien vorbereitet (`scripts/`, `.github/workflows/release.yml`, `release/catalog.pub`).

## Belegt (2026-11-19)
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
