# Daedyx Tasks veröffentlichen — was Björn selbst tun muss

Stand: 2026-11-19, Paket K3. Der Workflow `.github/workflows/release.yml` ist **nur als Datei vorbereitet**. Er ist nie gelaufen, es gibt keine Secrets, nichts ist veröffentlicht. Schlüssel und Passwörter tippt Björn selbst; sie gehören nie in den Chat oder in Git.

## 1. Update-Schlüssel für Tasks erzeugen (einmalig)
PowerShell im Ordner `app` dieses Repositorys:

```powershell
npx tauri signer generate -w "$env:USERPROFILE\.daedyx\tasks-update.key"
```

Das Passwort tippst du selbst. Danach:
1. Den privaten Schlüssel `tasks-update.key` an **zwei** Orten sichern (Dashlane). Nie in Git.
2. Den öffentlichen Teil (`tasks-update.key.pub`, eine Zeile) einem Bau-Chat geben. Er trägt ihn in `app/src-tauri/tauri.conf.json` bei `plugins.updater.pubkey` ein und stellt `bundle.createUpdaterArtifacts` auf `true` (ein Test im Kern prüft, dass beides zusammenpasst). Solange der Schlüssel leer ist, hängt die App den Updater gar nicht ein und die Seite „Über Daedyx Tasks“ meldet beim Suchen einen Fehler.

## 2. Secrets im Repository `daedyx/tasks` anlegen
GitHub → Repository `daedyx/tasks` → Settings → Secrets and variables → Actions → New repository secret:

| Secret | Inhalt |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | Inhalt der Datei `tasks-update.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | das Passwort dazu |
| `DAEDYX_CATALOG_KEY` | Katalog-Schlüssel, **derselbe Wert wie bei Desk** |
| `DAEDYX_KEY_PASSWORD` | Passwort des Katalog-Schlüssels (wie bei Desk) |
| `RELEASES_TOKEN` | Token mit Schreibrecht nur auf `daedyx/releases` (wie bei Desk) |
| `DAEDYX_READ_TOKEN` | **neu (früher `CORE_READ_TOKEN`, seit T1b):** Fine-grained Token, nur die Repositories `daedyx/core` und `daedyx/ui`, Berechtigung Contents: Read-only. Der Workflow braucht ihn, weil der Aufgaben-Kern und `daedyx-sys` aus privaten Repositories kommen. Dasselbe Token gilt für Desk und für `daedyx/ui`. |

Der Workflow prüft vor dem Bau, ob alle sechs da sind, und bricht sonst mit einer klaren Meldung ab.

## 3. Erster Lauf (erst auf dein Go)
GitHub → Actions → Release → Run workflow: Version (z. B. `0.1.0`, muss höher sein als die letzte im Katalog; für Tasks ist der Katalog noch leer) und eine Änderungsnotiz. Der Lauf baut, testet, prüft das Setup (Installationsprobe), lädt das Setup als Release `tasks-v<Version>` nach `daedyx/releases`, trägt `tasks` in den signierten Katalog ein und legt `tasks/latest.json` ab. Danach erscheint „Daedyx Tasks“ im Launcher.

## 4. Was ohne Veröffentlichung schon geht
- Lokal bauen: `cd app; npm run tauri build` (mit Strawberry-Perl-Pfad, siehe `STAND.md`). Setup: `app/src-tauri/target/release/bundle/nsis/Daedyx Tasks_<Version>_x64-setup.exe`.
- Lokalen Katalog des Launchers füllen: `scripts\release.ps1 -Version 0.1.0 -Notes "…"` (braucht den Katalog-Schlüssel unter `%LOCALAPPDATA%\Daedyx\keys\catalog.key`, den Desk schon nutzt; dasselbe Verfahren wie bei Desk, siehe dessen `docs/RELEASE.md`). Beendet vorher eine laufende Tasks-App.
- Trockenprobe der Veröffentlichung ohne Netz: `scripts\ci\test-publish.ps1 -CatalogTool <daedyx-catalog.exe> -LauncherCli <daedyx.exe>`.

## Bekanntes
- `publish.ps1` ergänzt `.gitattributes` im Katalog-Repository (Zeilenenden der signierten Dateien) jetzt **zusätzlich** zu vorhandenen Zeilen. Die Desk-Fassung des Skripts überschreibt die Datei dagegen und würde `tasks/latest.json` wieder entfernen: Sobald beide Apps veröffentlichen, gleiche Desk-`publish.ps1` an (Meldung an GH & Release).
- Beide Apps schreiben in denselben Katalog; die Pipelines serialisieren sich je Repository (`concurrency`), nicht untereinander. Zwei Läufe gleichzeitig (Desk und Tasks) sind durch die Wiederholung bei abgelehntem Push abgefangen, aber nicht praktisch erprobt.
