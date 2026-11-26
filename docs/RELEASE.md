# Daedyx Tasks veröffentlichen — was Björn selbst tun muss

Stand: 2026-11-27, T7-Vorbereitung (Zweig `tasks-rust-t7v`). Der Workflow `.github/workflows/release.yml` ist ein
**Entwurf**: Er ist nie gelaufen, und es ist nichts veröffentlicht. Er braucht **keine Secrets, keine Schlüssel und
keine Passwörter**. Signiert wird über einen Nachweis, den GitHub selbst ausstellt (Sigstore). Die App prüft genau
diesen Nachweis: Workflow `.github/workflows/release.yml`, Zweig `main`, Repository `daedyx/tasks` mit festen Kennungen
(`app/src-tauri/crates/tasks-app/src/update.rs`).

Die alte Anleitung (Tauri-Update-Schlüssel, Katalog-Schlüssel, `RELEASES_TOKEN`, `DAEDYX_READ_TOKEN`) gilt nicht mehr.
Für Tasks legst du **keinen** dieser Schlüssel und **kein** Token an.

## Bevor du anfängst: Was ein Bau-Chat noch erledigen muss (T7)
Diese Lücken stehen im Entwurf. Solange eine offen ist, bricht der Lauf ab, ohne etwas zu veröffentlichen.

| Lücke | Was fehlt |
|---|---|
| L1 | Die volle Commit-ID der Aktion `actions/attest-build-provenance`. Sie lag in der Vorbereitung nicht vor; dort stehen absichtlich nur Nullen. |
| L2 | Das Produkt-Setup `installer/tasks.nsi` (Name „Daedyx Tasks“, echte Kennungen). Bisher gibt es nur das Setup der Vorschau. |
| L3 | Der Programmname der Produkt-Fassung `daedyx-tasks.exe`. Heute heißt das Programm auch mit dem Merkmal `produkt` noch `daedyx-tasks-vorschau.exe`. |
| L4 | NSIS auf dem GitHub-Läufer: ob `makensis` dort schon liegt, ist **ungeprüft**. Sonst braucht es einen Installationsschritt mit fester Version und Prüfsumme. |
| L5 | Der Prüfschritt `gh attestation verify` (Schalter `--source-ref` und `--deny-self-hosted-runners`) ist **ungeprüft**. |

## Was du selbst tust, Schritt für Schritt
Jeder Schritt ist ein eigener Klick-Weg. Mach ihn erst, wenn du ausdrücklich Ja dazu gesagt hast. Die Menünamen bei
GitHub können leicht anders heißen (**ungeprüft**, Stand der Vorbereitung).

### Schritt 1: Die drei Repositories öffentlich stellen
Der Nachweis von GitHub funktioniert ohne Bezahl-Tarif nur in öffentlichen Repositories. Ohne Lese-Token muss der Bau
außerdem den Kern und die Oberflächen-Bausteine frei abrufen können. Erst nach der Verlaufsprüfung und deinem Ja.
1. Öffne auf GitHub das Repository `daedyx/core`.
2. Klicke auf **Settings**.
3. Scrolle ganz nach unten zu **Danger Zone**.
4. Klicke auf **Change visibility** und dann auf **Change to public**.
5. Bestätige, wie GitHub es verlangt.
6. Mach dasselbe für `daedyx/ui` und danach für `daedyx/tasks`.

### Schritt 2: Dem Workflow nur Leserechte als Standard geben
1. Im Repository `daedyx/tasks`: **Settings**, dann **Actions**, dann **General**.
2. Unter **Workflow permissions** wählst du **Read repository contents and packages permissions**.
3. Wenn es den Haken **Require actions to be pinned to a full-length commit SHA** gibt: setzen.
4. Klicke auf **Save**.

Der Release-Workflow bekommt die Rechte zum Veröffentlichen nur für sich selbst, nur für diesen Lauf.

### Schritt 3: Das Environment „release“ mit deiner Freigabe anlegen
So startet nichts, bevor du es erlaubst.
1. Im Repository `daedyx/tasks`: **Settings**, dann links **Environments**.
2. Klicke auf **New environment**.
3. Name: `release` (klein geschrieben, genau so). Klicke auf **Configure environment**.
4. Setze den Haken bei **Required reviewers** und trage dich selbst ein.
5. Lass **Prevent self-review** aus, sonst kannst du deinen eigenen Lauf nicht freigeben.
6. Bei **Deployment branches and tags** wählst du **Selected branches and tags** und fügst `main` hinzu.
7. Lege hier **keine** Secrets und **keine** Variablen an.
8. Klicke auf **Save protection rules**.

### Schritt 4 (empfohlen): Unveränderliche Releases einschalten
Dann lassen sich Tag und Dateien eines Releases nachträglich nicht mehr austauschen.
1. Im Repository `daedyx/tasks`: **Settings**, dann **General**.
2. Suche den Abschnitt **Releases** und schalte **Release immutability** ein (Name **ungeprüft**).

Hast du eine Regel, die nur dir erlaubt, Tags `v*` anzulegen: Der Workflow legt den Tag `v<Version>` selbst an.
Die Regel muss das zulassen, sonst scheitert der letzte Schritt (**ungeprüft**, beim ersten Lauf ansehen).

### Schritt 5: Version anheben lassen
Ein Bau-Chat hebt die Version in `app/src-tauri/crates/tasks-app/Cargo.toml` an, zum Beispiel auf `0.3.0`, und bringt
das auf `main`. Der Workflow schreibt nichts mehr in den Quellcode zurück. Er prüft nur, dass deine Eingabe dort steht.

### Schritt 6: Den Lauf starten
1. Im Repository `daedyx/tasks`: **Actions**, links **Release**.
2. Klicke auf **Run workflow**.
3. Bei **Branch** muss `main` stehen.
4. Trage die **Version** ein, zum Beispiel `0.3.0`.
5. Trage eine kurze **Änderungsnotiz** ein: eine Zeile, ohne Links, ohne echte Namen.
6. Tippe die Version bei **Bestätigung** noch einmal ein.
7. Klicke auf den grünen Knopf **Run workflow**.

### Schritt 7: Die Freigabe klicken
1. Öffne den Lauf, der jetzt oben in der Liste steht. Er wartet mit dem Hinweis „Waiting for review“.
2. Klicke auf **Review deployments**.
3. Setze den Haken bei **release** und klicke auf **Approve and deploy**.

Danach testet der Lauf, baut die Produkt-Fassung und das Setup, lässt GitHub beides bezeugen, prüft den Nachweis
selbst und legt das Release `v<Version>` an. Das dauert ungefähr eine Stunde (**ungeprüft**).

### Schritt 8: Nachsehen
1. Im Repository `daedyx/tasks`: rechts **Releases**.
2. Das neue Release `v<Version>` muss als **Latest** markiert sein und genau drei Dateien haben:
   `daedyx-tasks-<Version>-setup.exe`, `entry.json` und `provenance.sigstore.json`.
3. Wenn etwas fehlt oder rot ist: nichts löschen, einem Bau-Chat Bescheid sagen.

## Zwei Regeln für immer
- **Releases nie löschen, nur durch ein neueres ersetzen.** Wer ein Release gesehen hat, bekommt sonst dauerhaft
  „Quelle liefert ältere Version, bitte melden“ (Review U1-R, H-U1-3).
- **Kein Secret im Release-Weg.** Braucht ein neuer Schritt doch eines, entscheidest du das vorher schriftlich
  (plan-ohne-secrets (f)). Die CI meldet in `release.yml` jedes Secret als Fehler und in den übrigen Workflows jedes außer
  `DAEDYX_READ_TOKEN` (Lese-Token für CI und Mac-Testbau, bis `core` und `ui` öffentlich sind).

## Hinweise für Bau-Chats
- Die App-Kennung im Eintrag (`app`) ist `daedyx-tasks`; die Vorschau erwartet `daedyx-tasks-vorschau` und lehnt
  Produkt-Einträge ab.
- `entry.json` schreibt `scripts/ci/make-entry.ps1`. Der Test `release_tests.rs` (tasks-app) prüft den Workflow-Text
  gegen den Prüfweg des Kerns: Pfad, Zweig, Repository, Auslöser, keine Secrets, gepinnte Aktionen, Merkmal
  `produkt`. Er baut ein erfundenes Release mit dem Skript und den Dateinamen aus `release.yml` und schickt es
  durch Suche und Download.
- Die alten Skripte des Tauri-Wegs (`scripts/release.ps1`, `scripts/ci/publish.ps1`, `check-release-inputs.ps1`,
  `fetch-catalog-tool.ps1`, `scripts/lib/*catalog*`, `minisign*`) nutzt der neue Workflow nicht mehr. Sie fallen mit
  der Tauri-Hülle weg.
