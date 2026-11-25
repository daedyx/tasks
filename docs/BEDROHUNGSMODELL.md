# Bedrohungsmodell (Daedyx Tasks)

Vorlage: `SICHERHEIT.md` der Suite, Anhang G. Je Baustein ein Abschnitt, geschrieben **vor** dem Bau (SICHERHEIT 5.1).
Angreifer-Nummern G1–G10 nach SICHERHEIT 1.2.

## Baustein `daedyx-tasks-store` (2026-11-24, Zweig `tasks-rust-t0`)

Meilenstein T0 des Rust-Umbaus: Datenhaltung ohne Tauri. Inhalt verschoben aus der Tauri-Hülle, Verhalten gleich bis
auf die Härtungen unten.

**Werte:** Aufgaben, Listen, Abschnitte, Erinnerungen in der SQLCipher-Datenbank `daedyx-tasks.db` im Datenordner der
App (`%APPDATA%\de.daedyx.tasks`, Entwicklung `.dev`); Sicherungen in `backups\` daneben (gleicher Schlüssel); der
Datenbank-Schlüssel (64 Hex-Zeichen) im Windows-Tresor, Dienst `daedyx-tasks`, Eintrag `db-key`.

**Eingänge von außen:**
| Eingang | Herkunft | Grenze Größe | Grenze Tiefe/Anzahl | Zeit | Parser | Fuzz-Ziel |
|---|---|---|---|---|---|---|
| Datenbank-Datei | Datenordner (andere Version, Beschädigung, fremde Datei) | keine eigene (SQLite) | Schema-Stand ≤ Kern | Start | SQLCipher/SQLite | später `import_file` (T5) |
| Tresor-Wert `db-key` | Windows-Tresor | genau 64 Hex-Zeichen | 1 | Start | `is_valid` | – (Missbrauchstests) |
| Uhrzeit, Zeitzone für den Takt | Betriebssystem | Name: frei, Kern fällt auf UTC zurück | – | alle 30 s | Kern `zone` | – (Missbrauchstests) |

**Vertrauensgrenzen:** Store ↔ Dateisystem (nur der übergebene Pfad und `backups\` daneben); Store ↔ Tresor (nur
`SecretStore`); Store ↔ Hülle (Fehler nur als `StartupCode`, Texte macht die Oberfläche; Platzmessung kommt als
Funktion von außen, damit hier kein `unsafe` nötig ist).

**Missbrauchsfälle:**
| Nr. | Angreifer | Was passiert | Gegenmaßnahme | Testname |
|---|---|---|---|---|
| 1 | G1 feindliche Daten | Datei ist eine unverschlüsselte SQLite-Datei | Schlüsselprüfung vor jedem Schreiben, Code `WrongKey` | `a_plain_sqlite_file_is_not_sqlcipher_and_stays_unchanged` |
| 2 | G1/G7 | Datei ist leer (0 Byte) und würde sonst still zu einer neuen, leeren Datenbank | Abweisen, Code `Other`, Datei bleibt | `an_empty_file_is_refused_not_turned_into_a_new_database` |
| 3 | G1/G7 | Datei abgeschnitten (Kopierfehler, voller Datenträger) | SQLCipher meldet beim ersten Lesen Schlüssel- oder Formatfehler; Code, keine Sicherung, keine Migration | `a_truncated_database_is_refused_and_stays_unchanged` |
| 4 | G1 | Tresor-Wert kaputt (leer, zu lang, kein Hex, Umlaute) | `BadKey`, Wert wird nie überschrieben | `a_broken_key_in_the_vault_is_refused_and_kept` |
| 5 | – | Tresor nicht erreichbar | Abbruch vor dem Anlegen einer Datei | `an_unreachable_vault_is_refused_and_creates_no_file` |
| 6 | G8 | Fehlertext verrät Schlüssel oder Pfad | Fehlertexte nur Fehlerart | `the_error_text_names_no_key_and_no_path` |
| 7 | G1 | Uhr oder Zeitzone unsinnig (negativ, `i64::MAX`, lange Namen) | Takt prüft den Zeitraum vor dem Kern, nie Panik | `hostile_clock_and_zone_values_do_not_panic` |
| 8 | – | Panik an anderer Stelle hält die Datenbank-Sperre | Durchgang fällt still aus | `a_poisoned_lock_skips_the_tick_without_panic` |
| 9 | G1/G7 | Unsinnige Werte in der Datenbank (Erinnerung `i64::MAX` Minuten, Fälligkeit `i64::MIN`): der Kern-Durchgang läuft über, mit `panic = "abort"` Absturz bei jedem Start (Review T0, M-1) | eigener, geprüft rechnender Durchgang im Store; Minuten nur 0–525 600, Fälligkeit nur 1970–9999, ungültige Erinnerungen übersprungen, Datenbank unverändert | `absurd_reminder_minutes_in_the_database_are_skipped_without_panic`, `an_absurd_due_time_in_the_database_is_skipped_without_panic`, `a_broken_task_does_not_hide_the_valid_ones`, `the_checked_scan_matches_the_core_for_valid_data` |
| 10 | G1/G7 | Zeile mit falschem Datentyp (Titel als Binärwert, Minuten als Text) schaltet alle Erinnerungen still ab (Review T0, N-7) | Zeilen einzeln lesen, kaputte überspringen; Aufgabe mit nur kaputten Erinnerungszeilen fällt nicht auf das alte Einzelfeld zurück | `a_row_with_a_wrong_data_type_does_not_silence_the_other_reminders` |
| 11 | G8 | Späterer Code ruft doch den ungeprüften Kern-Durchgang auf | Hülle reicht `reminders` des Kerns nicht weiter; Wächter mit Gegenprobe | `nobody_reaches_the_unchecked_core_tick`, `the_guard_sees_every_way_in` |

**Netz:** keins. Der Baum des Store-Crates steht als Erlaubnisliste im Test `guard_tests` (geschlossene Welt, Review T0 N-5).

**Löschen:** Der Store löscht nur eigene alte Sicherungen (`daedyx-tasks-v<n>.db`, die drei neuesten bleiben), nie die
Datenbank, nie fremde Dateien im Sicherungsordner, nie einen Tresor-Wert.

**Nicht geschützt:** Schadsoftware mit Benutzerrechten kann Tresor und Datei lesen (G9). Die Tauri-Hülle steht nicht
unter den strengen Lints (Feature-Stopp, Eintrag in `SICHERHEIT-AUSNAHMEN.md`). Die beiden `unsafe`-Stellen (freier
Platz, Meldungsfenster) bleiben bis T1 in der Hülle.

**Offen/ungeprüft:** Schlüssel liegt als `String` im Speicher, ohne `zeroize` (SICHERHEIT 2.10, kommt mit dem
`Secret`-Typ); `Connection::open` außerhalb des Store-Crates ist noch nicht per Clippy gesperrt (Plan 4.2, T5); die
Kennungen der Vorschau (Plan 4.2) kommen mit T2/T5.

## T4 Hintergrund und Einstellungen (2026-11-25)

Neue Grenzen der Rust-Vorschau: andere Prozesse desselben Benutzers (Pipe, Befehlszeile) und Windows (Benachrichtigung,
Infobereich, Autostart in HKCU). Weiter kein Netz, keine Datei, kein Tresor.

| Nr. | Bedrohung | Gegenmaßnahme | Test |
|---|---|---|---|
| T4-1 | G9: Aufgabentitel im Info-Center und auf dem Sperrbildschirm | Benachrichtigung neutral als Standard, Titel nur per Schalter | `benachrichtigung_ohne_titel_standard`, `schalter_standard_infobereich_an_titel_aus` |
| T4-2 | Anderer Prozess schickt Unsinn über die Pipe | nur `SHOW`/`QUIT`, höchstens 16 Byte, 2 s Zeitgrenze, keine Fernverbindungen (daedyx-background) | `zweiter_start_holt_das_fenster_und_unsinn_wirkt_nicht` |
| T4-3 | Vorschau schreibt unter den Kennungen der echten App (Autostart, App-Kennung, Pipe) | alle Namen aus `ids.rs` hinter der Weiche `produkt`; Startprobe und Programmtest | `kennung_und_tresor_gehoeren_zur_selben_fassung`, `tests/kennung.rs` |
| T4-4 | Test hinterlässt Autostart-Werte | Registry-Test nur unter `HKCU\Software\DaedyxTasksVorschauTest\<pid>`, danach gelöscht | `autostart_ein_aus_und_blockiert_unter_testschluessel` |
| T4-5 | Schließen ohne Infobereich verliert getippten Text | Entwurf wird vor dem Beenden übernommen, sonst bleibt die App offen | `schalter_aus_schliessen_beendet_nach_dem_entwurf` |
| T4-6 | Unsinnige Uhrzeit oder Fälligkeit bringt den Takt zum Absturz | geprüfter Durchgang des Stores, Uhr nur 1970–9999 | `uhr_ausserhalb_des_bereichs_meldet_nichts` |
| T4-7 | Feinstellung macht Schrift unlesbar | Regler begrenzt, Farben aus den Tokens, Kontrast-Test für jede Stellung | `jede_stellung_erfuellt_die_kontrastregeln` (daedyx/ui) |

**Nicht geschützt:** Ein Prozess desselben Benutzers kann `QUIT` schicken (Pipe-Rechte „nur dieser Benutzer“ offen
wie in Desk). Was Windows mit gezeigten Benachrichtigungen macht (Info-Center), liegt außerhalb der App; `--cleanup`
entfernt die eigenen.
