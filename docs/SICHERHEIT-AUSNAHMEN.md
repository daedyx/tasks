# Sicherheits-Ausnahmen (Tasks)

Regelwerk: `SICHERHEIT.md` (Daedyx-Suite), Abschnitt 2 und Anhang B. Jede Stelle, die eine Regel ausdrücklich
abschaltet oder lockert, steht hier: `#[allow(…)]`/`#[expect(…)]` (auch in `cfg_attr`), `#[ignore]`, `unsafe`,
`cfg`-Bedingungen, die Code vor dem Clippy der CI verstecken, Build-Skripte, Manifest-Einstellungen außerhalb der
Erlaubnisliste, Lockerungen in `deny.toml`, und befristet erlaubte Scan-Funde (`scan:<regel>`). Ohne Eintrag ist die
Ausnahme nicht erlaubt. `cargo xtask exceptions` (im Ordner `app/src-tauri`) zählt die Stellen und vergleicht **Datei
und Regel** mit dieser Tabelle; eine Stelle ohne passende Zeile, eine Zeile ohne Stelle oder ein abgelaufener Eintrag
sind rot. Das Werkzeug ist eine Kopie des Desk-Werkzeugs (gleicher Aufbau: Workspace unter der Repo-Wurzel).

**Alle Einträge sind Vorschläge des Bau-Chats T0 (2026-11-24, A-026 nach dem Review): Ja des Betreuers 2026-11-24.** Die meisten entsprechen
Einträgen, die der Betreuer für Desk am 2026-11-24 angenommen hat (in Klammern genannt). Die Tauri-Hülle steht nicht
unter den strengen Lints (wie Desk, Entscheidung E6; Eintrag A-007) und fällt mit dem Wechsel auf die Rust-Fassung weg.

**Tasks T1b (2026-11-25):** A-001 und A-011 (`free_space.rs` mit `unsafe`) sind entfallen; die Hülle misst den freien Platz
jetzt mit `daedyx-sys` aus `daedyx/ui` (dort Ausnahme A-020). Neu ist A-027 für die Git-Quelle (Ja ausstehend). Die Nummern
bleiben frei.

| Nr. | Datei | Regel | Grund | Entscheid von | Datum | Ablauf |
|---|---|---|---|---|---|---|
| A-002 | app/src-tauri/src/startup_dialog.rs | unsafe (`unsafe extern "system"`) | Meldungsfenster `MessageBoxW` für Startfehler, bevor eine Oberfläche läuft (Bestand der Hülle). Ersatz durch das Startfehler-Fenster in egui (T2) bzw. `daedyx-sys` (T1). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-002 | app/src-tauri/src/startup_dialog.rs | unsafe (Aufruf mit `// SAFETY:`) | wie A-002: der Aufruf selbst | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-003 | app/src-tauri/crates/tasks-store/src/secrets/tests.rs | ignore | Probe gegen den echten Windows-Tresor mit eigenem Test-Eintrag (nie `db-key`), nur von Hand. Dauerhaft (Desk A-004). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-004 | app/src-tauri/xtask/src/files.rs | clippy::disallowed_methods | Das Prüfwerkzeug startet `git` mit festen Argumenten, um die versionierten, vorgemerkten oder im Commit stehenden Dateien zu lesen. Keine Shell, keine Fremddaten außer Dateinamen aus Git (Desk A-019). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-005 | app/src-tauri/crates/tasks-store/src/test_support.rs | clippy::disallowed_methods | `test_dir` legt Wegwerf-Ordner für Tests im System-Temp an (`std::env::temp_dir`), nie im Datenordner der App. Nur mit `cfg(test)` oder Merkmal `test-support` übersetzt. Eine Stelle statt einer Freigabe für das ganze Crate (Desk A-005 gibt die Regel im ganzen Testbau frei). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-006 | app/src-tauri/xtask/src/main.rs | clippy::print_stdout, clippy::print_stderr | Das Prüfwerkzeug meldet Datei, Zeile und Regel auf der Konsole, nie den gefundenen Text (Desk A-020). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-007 | app/src-tauri/Cargo.toml | manifest:lints (Tauri-Hülle ohne Workspace-Lints) | Die alte Tauri-Oberfläche steht nicht unter den strengen Regeln (Feature-Stopp, Desk E6/A-021); sie fällt mit dem Wechsel auf die Rust-Fassung weg. | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-008 | app/src-tauri/build.rs | build.rs | Tauri-Build-Skript (`tauri_build::build()`): erzeugt Kontext und Windows-Ressourcen der alten Hülle. Fällt mit der Hülle weg (Desk A-024). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-009 | app/src-tauri/src/lib.rs | unused_mut | Aus der Tauri-Vorlage: `builder` wird nur auf dem Desktop bzw. mit `dev-bridge` verändert. Ohne Sicherheitswirkung; fällt mit der Hülle weg (Desk A-026). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-010 | app/src-tauri/src/lib.rs | cfg:desktop | Tauri setzt `desktop` über sein Build-Skript; unter Windows ist es wahr, Clippy sieht den Code also. Das Werkzeug kennt den Namen nicht (Desk A-027). Erste von vier Stellen. | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-010 | app/src-tauri/src/lib.rs | cfg:desktop | wie A-010, zweite Stelle | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-010 | app/src-tauri/src/lib.rs | cfg:desktop | wie A-010, dritte Stelle | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-010 | app/src-tauri/src/lib.rs | cfg:desktop | wie A-010, vierte Stelle | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-011 | app/src-tauri/src/startup_dialog.rs | cfg:not(windows) (`show_error`) | Platzhalter für Nicht-Windows: Ausgabe auf der Konsole statt Fenster; wie A-011. | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-012 | app/src-tauri/deny.toml | bans.allow-wildcard-paths=true | Eigene Crates im Workspace hängen nur per Pfad voneinander ab (`publish = false`); fremde Crates bleiben von `wildcards = "deny"` erfasst (Desk A-032). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-013 | app/src-tauri/deny.toml | RUSTSEC-2026-0459 (libsqlite3-sys/SQLite) | Lücke nur mit präparierter DB-/WAL-Datei; SQLCipher bündelt auch in 0.39 ein betroffenes SQLite (Kern A-004, Desk A-009). Neu prüfen, sobald sqlcipher-src SQLite >= 3.54 enthält. | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-014 | app/src-tauri/deny.toml | RUSTSEC-2026-0543 (chrono, nicht gepflegt) | keine bekannte Lücke; Ersatz ist ein eigener Auftrag (Kern A-005, Desk A-010) | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-015 | app/src-tauri/deny.toml | RUSTSEC-2026-0555 (chrono-tz, nicht gepflegt) | keine bekannte Lücke; Ersatz zusammen mit chrono (Kern A-006, Desk A-011) | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-016 | .github/workflows/release.yml | scan:secret-im-workflow | Alter Release-Ablauf der Tauri-Hülle mit Signaturschlüssel, Passwort und Token als GitHub-Secrets (Bestand, Desk A-033). Wird mit dem Plan „ohne Secrets“ und dem Wechsel auf die Rust-Fassung abgelöst. Anders als in Desk bricht er auf dem Rust-Zweig noch nicht im ersten Schritt ab (Workflow in T0 nicht geändert). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-017 | .github/workflows/mac-test.yml | scan:secret-im-workflow | Mac-Testbau liest den privaten Kern mit `CORE_READ_TOKEN` (nur Lesen, nur im Abruf-Schritt). Entfällt, sobald `daedyx/core` öffentlich ist (wie Desk A-043). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-018 | scripts/lib/minisign.mjs | scan:geheimnis | Erzeugt das Dateiformat eines minisign-Schlüssels (Kopfzeile „secret key“); der Schlüssel selbst steht nie im Repo (Desk A-041). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-019 | scripts/lib/minisign.test.mjs | scan:geheimnis | Test des Formats aus A-018 mit einem nur im Test erzeugten Wegwerf-Schlüssel (Desk A-042). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-020 | app/src-tauri/deny.toml | RUSTSEC-2026-0594 (cookie, nur Tauri-Hülle) | Nonces wiederholen sich nur nach `fork()`; Windows laut Meldung nicht betroffen, Tauri nutzt keine privaten Cookies. Behoben in 0.18.3, kommt mit dem nächsten Tauri-Update (wie Desk). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-021 | app/src-tauri/deny.toml | RUSTSEC-2024-0370 (proc-macro-error, nur Tauri-Hülle) | nicht gepflegt, nur Bauzeit-Makro über gtk/glib (Linux); unter Windows nicht im Bau (wie Desk). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-022 | app/src-tauri/deny.toml | RUSTSEC-2026-0370 (zvariant, nur Tauri-Hülle) | nicht gepflegt, nur Linux-D-Bus über `tauri-plugin-notification` (wie Desk, Launcher A-020). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-023 | app/src-tauri/deny.toml | RUSTSEC-2026-0371 (zbus_names, nur Tauri-Hülle) | wie A-022 (Launcher A-021). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-024 | app/src-tauri/deny.toml | RUSTSEC-2026-0372 (zvariant_derive, nur Tauri-Hülle) | wie A-022 (Launcher A-022). | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-025 | app/src-tauri/deny.toml | RUSTSEC-2024-0429 (glib, nur Tauri-Hülle) | Unsoundness in `VariantStrIter`; `glib` kommt nur über gtk (Linux-Infobereich von Tauri, Merkmal `tray-icon`), unter Windows nicht im Bau, die Hülle ruft es nicht auf. Neu in Tasks (Desk hat kein `tray-icon`). Fällt mit der Hülle weg. | Bau-Chat T0, Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-03-31 |
| A-026 | app/src-tauri/deny.toml | graph.targets=["x86_64-pc-windows-msvc", "aarch64-apple-darwin", "x86_64-apple-darwin"] | `cargo deny` prüft nur die Systeme, für die Tasks gebaut wird (Windows; macOS im Mac-Testbau). So braucht die Prüfung keine Quellen für Linux, Android oder iOS (Review T0, N-3: keine Downloads für die Prüfung). Folge: Meldungen, die nur Linux-Teile betreffen (A-021 bis A-025), sieht `cargo deny` nicht mehr; `cargo audit` prüft weiter den ganzen Lockfile. | Bau-Chat T0 (Auftrag Master nach Review), Ja des Betreuers 2026-11-24 | 2026-11-24 | 2027-05-31 |
| A-027 | app/src-tauri/deny.toml | sources.allow-git=https://github.com/daedyx/ui | `daedyx-sys` (Platzmessung vor der Sicherung, einzige `unsafe`-Stelle der Daedyx-Apps) kommt aus dem eigenen privaten Repository `daedyx/ui`, wie der Kern aus `daedyx/core` und wie in Desk (A-045). Bis zum nächsten ui-Tag per `rev` auf Zweig `paket-t1b`. Ersetzt die eigene `free_space.rs` mit `unsafe` (A-001 und A-011 entfallen). Die Release-Pipeline braucht dafür noch Lesezugriff auf `daedyx/ui` (offen, wie in Desk). | Bau-Chat T1b, Ja des Betreuers ausstehend | 2026-11-25 | 2027-05-31 |

Eine Zeile mit Regel `scan:<regel>` erlaubt genau diesen Fund des Scans in genau dieser Datei bis zum Ablaufdatum; sie
zählt nicht als Code-Stelle. Format für `cargo xtask exceptions`: Spalte 2 ist der Pfad ab der Repo-Wurzel, Spalte 3 die
Regel (Text vor einer Erläuterung in Klammern), Spalte 6 das Eintragsdatum und Spalte 7 der Ablauf, beide `JJJJ-MM-TT`,
Ablauf höchstens 12 Monate nach dem Eintrag. Eine Datei mit zwei gleichen Stellen bekommt zwei Zeilen. Einzelheiten zu
den Regeln: `docs/SICHERHEIT-AUSNAHMEN.md` im Kern-Repository (`daedyx/core`).

## Stand der Prüfung

Seit 2026-11-24 (T0) ist `cargo clippy -p daedyx-tasks-store -p xtask --all-targets --all-features -- -Dwarnings`
grün. Das Store-Crate hat kein `unsafe`, kein `#[allow]` außer A-005 und kein `cfg`, das Code vor Clippy versteckt.
