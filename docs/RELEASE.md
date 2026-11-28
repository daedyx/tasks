# Daedyx Tasks veröffentlichen — was Björn selbst tun muss

Stand: 2026-11-27, T7 releasefertig (Zweig `tasks-rust-t7v`, noch nicht auf `main`). Der Workflow
`.github/workflows/release.yml` ist nie gelaufen, und es ist nichts veröffentlicht. Er braucht **keine Secrets, keine Schlüssel und
keine Passwörter**. Signiert wird über einen Nachweis, den GitHub selbst ausstellt (Sigstore). Die App prüft genau
diesen Nachweis: Workflow `.github/workflows/release.yml`, Zweig `main`, Repository `daedyx/tasks` mit festen Kennungen
(`app/src-tauri/crates/tasks-app/src/update.rs`).

Die alte Anleitung (Tauri-Update-Schlüssel, Katalog-Schlüssel, `RELEASES_TOKEN`, `DAEDYX_READ_TOKEN`) gilt nicht mehr.
Für Tasks legst du **keinen** dieser Schlüssel und **kein** Token an.

## Bevor du anfängst: Was ein Bau-Chat noch erledigen muss (T7)
L1, L2, L3 und L6 sind geschlossen. L4 und L5 sind nur noch beim ersten Lauf zu belegen: Scheitern sie, bricht der
Lauf ab, ohne etwas zu veröffentlichen. L5b hält den Lauf nicht an; beim ersten Lauf ansehen.

| Lücke | Was fehlt |
|---|---|
| L1 | **Geschlossen** (2026-11-27, nach Björns Ja zum Nachschlagen): `actions/attest-build-provenance` v4.2.2 = `4d101475d8b20a2381f78447822ac1eab6504dd8` (Release vom 2026-08-06). Sie ruft `actions/attest` selbst per Commit-ID auf: `508db95dd578ae2727ebd6217d5ba78e4fbda05d` (v4.2.1). Ob diese Fassung Rekor-v1-Einträge schreibt, wie die App sie verlangt, ist **ungeprüft**; das zeigt erst die Selbstprüfung im ersten Lauf (sie bricht sonst vor dem Veröffentlichen ab). Der Wächter-Test lehnt den Platzhalter aus Nullen jetzt ab (H-T7v-3). |
| L2 | **Geschlossen** (2026-11-27): Produkt-Setup `installer/tasks.nsi` mit „Daedyx Tasks“, `de.daedyx.tasks`, `daedyx-tasks.exe`, dieselbe Vorlage wie die Vorschau. Uninstall-Schlüssel `Daedyx Tasks`, genau der Name, den der Launcher sucht (F-U3-5, Test `setup_produkt_tests.rs`). Lokal mit makensis gebaut, aber **nicht installiert** (die echte App wird auf Björns Rechner nie angefasst). |
| L3 | **Geschlossen** (2026-11-27): eigenes Programmziel `daedyx-tasks` (`src/main_produkt.rs`, nur mit `produkt`); der Lauf baut es mit `--bin daedyx-tasks`. `tests/kennung.rs` prüft mit `produkt` genau dieses Programm. |
| L4 | NSIS auf dem GitHub-Läufer: ob `makensis` dort schon liegt, ist **ungeprüft** (nach meinem Wissen gehört NSIS zum Abbild `windows-2025`, nicht nachgeschlagen). Fehlt es, bricht „Setup bauen“ im Job „bauen“ ab, vor jeder Freigabe; dann braucht es einen Installationsschritt mit fester Version und Prüfsumme. |
| L5 | Alle Schalter von `gh attestation verify` (`--bundle`, `--repo`, `--signer-workflow`, `--source-ref`, `--deny-self-hosted-runners`) gibt es laut Hilfe von gh 2.103.2 (lokal gelesen, 2026-11-27). Mit `--bundle` liest gh den Nachweis aus der Datei; ob das Token mit nur `contents: read` im Job „pruefen“ reicht, zeigt erst der erste Lauf (sonst kein Release). |
| L5b | Tag-Regel: Erlaubt nur dir eine Regel, Tags `v*` anzulegen, scheitert `gh release create` womöglich erst beim Tag, nach dem Hochladen. Dann kann ein **Entwurfs-Release** liegen bleiben (**ungeprüft**). Beim ersten Lauf ansehen und nur einen solchen Entwurf entfernen. |
| L6 | **Geschlossen** (2026-11-27): vier Jobs, jeder nur mit den Rechten für seine Aufgabe (Review T7v, N-T7v-4 Weg 1 und M-T7v-3). „bauen“ und „pruefen“ nur mit Leserecht, „bezeugen“ mit Freigabe und Kennung für den Nachweis, „veroeffentlichen“ nur mit `contents: write`. Programme aus dem Bau (das Prüfprogramm) laufen nur in „pruefen“, ohne Kennung und ohne Schreibrecht; die Jobs mit Rechten führen nichts aus der Übergabe aus. Übergabe mit `actions/upload-artifact` v7.0.2 = `cf430e030ddbb5b0abf93d22962f4752f3646cd9` (2026-10-07) und `actions/download-artifact` v8.0.2 = `9000827ccba6bdab643e8b6fd33ac0654aef8333` (2026-10-07), Prüfsummen aus dem Bau-Job werden gegengeprüft. |

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
Das erste Produkt-Release ist **0.2.0** (Entscheid T-F8: keine Verwechslung mit den Tauri-Kandidaten 0.1.0); diese
Version steht schon in `Cargo.toml`. Für jedes spätere Release hebt ein Bau-Chat die Version in
`app/src-tauri/crates/tasks-app/Cargo.toml` an, zum Beispiel auf `0.3.0`, und bringt
das auf `main`. Der Workflow schreibt nichts mehr in den Quellcode zurück. Er prüft nur, dass deine Eingabe dort steht.

### Schritt 6: Den Lauf starten
1. Im Repository `daedyx/tasks`: **Actions**, links **Release**.
2. Klicke auf **Run workflow**.
3. Bei **Branch** muss `main` stehen.
4. Trage die **Version** ein, beim ersten Mal `0.2.0`.
5. Trage eine kurze **Änderungsnotiz** ein: eine Zeile, ohne Links, ohne echte Namen. Tippe sie am besten selbst;
   kopierter Text kann unsichtbare Zeichen enthalten, dann bricht der Lauf mit einer Meldung ab.
6. Tippe die Version bei **Bestätigung** noch einmal ein.
7. Klicke auf den grünen Knopf **Run workflow**.

### Schritt 7: Die Freigabe klicken
Der Lauf testet und baut zuerst, ohne Schreibrechte (ungefähr eine Stunde, **ungeprüft**). Danach wartet er auf dich.
1. Öffne den Lauf, der jetzt oben in der Liste steht. Er wartet mit dem Hinweis „Waiting for review“.
2. Klicke auf **Review deployments**.
3. Setze den Haken bei **release** und klicke auf **Approve and deploy**.

Danach laufen drei Jobs nacheinander, ohne weiteren Klick:
1. **Bezeugen:** Der Lauf prüft, dass die gebauten Dateien unverändert angekommen sind, und lässt GitHub Eintrag und
   Setup bezeugen.
2. **Prüfen:** Er prüft das Ergebnis genau so, wie jede installierte App es prüfen wird (Prüfweg des Kerns mit
   eingebauter Vertrauenswurzel), dazu noch mit `gh attestation verify`. Dieser Job hat keine Schreibrechte.
3. **Veröffentlichen:** Erst wenn beides passt, legt er das Release `v<Version>` an.

Schon vor dem Bau bricht er ab, wenn die Version nicht neuer ist als das bisherige neueste Release.

Deine Freigabe sitzt vor dem Bezeugen, weil dort die Kennung für den Nachweis entsteht (Empfehlung aus Review T7v).
Willst du lieber erst kurz vor dem Veröffentlichen klicken, sag es einem Bau-Chat; dann wandert `environment: release`
zum Job „veroeffentlichen“.

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
  `GITHUB_TOKEN`. `core` und `ui` sind öffentlich, CI und Mac-Testbau brauchen kein Lese-Token.

## Hinweise für Bau-Chats
- Die App-Kennung im Eintrag (`app`) ist `daedyx-tasks`; die Vorschau erwartet `daedyx-tasks-vorschau` und lehnt
  Produkt-Einträge ab.
- `entry.json` schreibt `scripts/ci/make-entry.ps1` (im Workflow mit Windows PowerShell 5.1, wie im Test). Es lehnt
  dieselben Notizen ab wie der Kern (`text::plain`). Vor dem Veröffentlichen prüft
  `examples/release_selbstpruefung.rs` (tasks-app, Merkmal `produkt`) die drei Dateien mit dem echten Prüfer der App
  (`release_check::verify`). Der Test `release_tests.rs` (tasks-app) prüft den Workflow-Text
  gegen den Prüfweg des Kerns: Pfad, Zweig, Repository, Auslöser, keine Secrets, gepinnte Aktionen, Merkmal
  `produkt`. Er baut ein erfundenes Release mit dem Skript und den Dateinamen aus `release.yml` und schickt es
  durch Suche und Download.
- Die alten Skripte des Tauri-Wegs (`scripts/release.ps1`, `scripts/ci/publish.ps1`, `check-release-inputs.ps1`,
  `fetch-catalog-tool.ps1`, `scripts/lib/*catalog*`, `minisign*`) nutzt der neue Workflow nicht mehr. Sie fallen mit
  der Tauri-Hülle weg.
