# Pflichtenheft-Matrix Rust-Fassung (Plan 1.3)

Erzeugt im Auftrag T2 (2026-11-25) aus den Tests der heutigen Oberfläche: 194 Tests in 34 Dateien.
Jede Zeile ist ein Verhalten (der Testname). Bis T7 bekommt jede Zeile einen Rust-Test (kittest gegen den echten
Kern) oder eine begründete Streichung; „Stand“ wird dann auf den Rust-Test oder „gestrichen: Grund“ gesetzt.
Meilensteine nach `plan-tasks-rust.md`: T2 Gerüst, T3a Liste und Detail, T3b Leiste und Board, T4 Hintergrund und
Einstellungen. „–“ heißt: entfällt mit Tauri/React (Plan 2.4) oder mit den festen Breiten im Stil 1.

**In T2 abgedeckt (eigene Rust-Tests, nicht 1:1 aus TypeScript):** Rahmen mit Navigationsleiste, Kopf- und Statusleiste
(Rollen und Namen), Start mit „Heute“, Seitenwechsel Liste/Board/Einstellungen, Sprache de/en/fr umschalten,
Leiste schmal und unten, Startfehler-Fenster je Fehlercode in drei Sprachen, Wächter für Texte, Farben und Kennung
(`daedyx-tasks-app`, `src/app_tests.rs`, `src/startfehler_tests.rs`, `src/guard_tests.rs`, `tests/`). Die
Zeilen aus `App.test.tsx`, `i18n.test.ts` und `useTheme.test.ts` hake ich erst ab, wenn ihr Verhalten Zeile für
Zeile verglichen ist (offene Frage im Bericht T2).

| Datei | Verhalten (Testname) | Meilenstein | Anmerkung | Stand |
|---|---|---|---|---|
| `App.test.tsx` | startet direkt im Aufgabenbereich | T2 | Rahmen, Start, Seitenwechsel | offen |
| `App.test.tsx` | legt eine Liste an, schreibt eine Aufgabe und hakt sie ab | T2 | Rahmen, Start, Seitenwechsel | offen |
| `App.test.tsx` | öffnet und schließt die Einstellungen (Darstellung und Updates) | T2 | Rahmen, Start, Seitenwechsel | offen |
| `App.test.tsx` | wechselt das Design und merkt es sich | T2 | Rahmen, Start, Seitenwechsel | offen |
| `background/backgroundPrefs.test.ts` | ist ohne gespeicherten Wert an (Erinnerungen sollen nicht am geschlossenen Fenster scheitern) | T4 | Einstellungen in task_meta (Plan 4.4) | offen |
| `background/backgroundPrefs.test.ts` | speichert und liest den Wert | T4 | Einstellungen in task_meta (Plan 4.4) | offen |
| `background/backgroundPrefs.test.ts` | fällt bei gesperrtem Speicher auf an zurück | T4 | Einstellungen in task_meta (Plan 4.4) | offen |
| `background/BackgroundSettings.test.tsx` | „Im Infobereich weiterlaufen“ ist standardmäßig an und lässt sich abschalten | T4 | Start und Hintergrund | offen |
| `background/BackgroundSettings.test.tsx` | zeigt den Autostart-Zustand und schaltet ihn um | T4 | Start und Hintergrund | offen |
| `background/BackgroundSettings.test.tsx` | meldet einen Fehler beim Autostart als Text und lässt den Zustand unverändert | T4 | Start und Hintergrund | offen |
| `background/BackgroundSettings.test.tsx` | ohne Tauri (Vorschau im Browser) ist der Autostart nicht schaltbar | T4 | Start und Hintergrund | offen |
| `background/useBackgroundRun.test.tsx` | legt das Symbol im Infobereich an, mit den Beschriftungen aus de.json | T4 | Hintergrund, Infobereich-Schalter (T-F9) | offen |
| `background/useBackgroundRun.test.tsx` | Schließen versteckt das Fenster statt zu beenden, solange die Einstellung an ist | T4 | Hintergrund, Infobereich-Schalter (T-F9) | offen |
| `background/useBackgroundRun.test.tsx` | Schließen beendet normal, wenn die Einstellung aus ist (Wert wird erst beim Schließen gelesen) | T4 | Hintergrund, Infobereich-Schalter (T-F9) | offen |
| `background/useBackgroundRun.test.tsx` | Öffnen und Beenden aus dem Menü laufen über die Schnittstelle | T4 | Hintergrund, Infobereich-Schalter (T-F9) | offen |
| `background/useBackgroundRun.test.tsx` | räumt beim Abbau auf | T4 | Hintergrund, Infobereich-Schalter (T-F9) | offen |
| `i18n/i18n.test.ts` | liefert den Text zum Schlüssel | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | ersetzt Platzhalter | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | gibt bei unbekanntem Schlüssel den Schlüssel zurück | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | enthält jeden im Code verwendeten Schlüssel (auch in Backticks und Schablonen) | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | erkennt Schlüssel in Backticks und Schablonen (Selbsttest der Wache) | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | stehen in keiner Komponente fest im JSX (alles über t()) | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `i18n/i18n.test.ts` | erkennt feste Texte im JSX, aber keine Symbole und keine t()-Aufrufe (Selbsttest der Wache) | T2 | Sprachdateien de/en/fr, Wächter „jeder Schlüssel“ | offen |
| `settings/DarkStyleSection.test.tsx` | zeigt drei Regler mit den aktuellen Werten | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `settings/DarkStyleSection.test.tsx` | meldet Änderungen als Zahlen je Feld | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `settings/DarkStyleSection.test.tsx` | zeigt sechs Vorlagen, markiert die passende und setzt Farbton und Tönung | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `settings/UpdatesSettings.test.tsx` | zeigt die Version und sucht beim Öffnen nach Updates | T4 | Updates in der App (T-F7) | offen |
| `settings/UpdatesSettings.test.tsx` | ohne Update-Schlüssel steht "Updates sind noch nicht eingerichtet" statt eines Fehlers | T4 | Updates in der App (T-F7) | offen |
| `settings/UpdatesSettings.test.tsx` | „Nach Updates suchen“ sucht erneut | T4 | Updates in der App (T-F7) | offen |
| `settings/UpdatesSettings.test.tsx` | zeigt eine neue Version mit Änderungen und installiert sie mit Fortschrittsbalken | T4 | Updates in der App (T-F7) | offen |
| `settings/UpdatesSettings.test.tsx` | zeigt Fehler als Text | T4 | Updates in der App (T-F7) | offen |
| `settings/UpdatesSettings.test.tsx` | die Einstellung „Beim Start nach Updates suchen“ ist standardmäßig an und lässt sich abschalten | T4 | Updates in der App (T-F7) | offen |
| `styles/surfaces.test.ts` | jedes Flächen-Token steht genau dreimal: hell, dunkel per Windows, dunkel per Umschalter | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | beide dunklen Blöcke tragen dieselben Werte | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Hellmodus: Bereichsleiste und Ordnerbaum sind getönt, die Liste ist am hellsten, die Kopfzeile hebt sich von der Lesefläche ab | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Dunkelmodus: Leiste am dunkelsten, dann Ordnerbaum, Liste, Lesefläche | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Text und gedämpfter Text bleiben auf allen Flächen lesbar (≥ 4,5:1), hell und in allen Dunkel-Stilen | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | gesperrte Knöpfe: Schrift auf ihrer Fläche und auf Leisten und Listen lesbar | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Hauptknopf mit Akzentfläche: Schrift darauf lesbar (arbeitend: Sättigung leicht reduziert) | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | kräftige Auswahl (Akzent über der Liste) trägt Text mit ≥ 4,5:1 in allen acht Bereichsfarben | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Bereichsleiste, Kopfleiste und Schreibfenster-Kopf (Akzent-getönte Leiste) tragen Text und gedämpften Text in allen Bereichsfarben | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/surfaces.test.ts` | Mail-Papier ist ein warmes Hell/Grau (nicht reinweiß) und trägt die Mailschrift mit hohem Kontrast | T2 | Flächen aus Tokens (daedyx-ui) | offen |
| `styles/tokens.test.ts` | die dunklen Hintergründe sind in beiden dunklen Blöcken über Farbton, Tönung und Helligkeit einstellbar | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `styles/tokens.test.ts` | keine Hex-Farben außerhalb von tokens.css | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `styles/tokens.test.ts` | dunkle Werte stehen für Windows-Einstellung und für den Umschalter | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `styles/tokens.test.ts` | color-scheme folgt dem Thema (native Bedienelemente) | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `styles/tokens.test.ts` | jede Palettenfarbe steht in allen drei Blöcken (hell, dunkel per Windows, dunkel per Umschalter) | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `styles/tokens.test.ts` | Schrift auf Kalenderfarben und Schriftfarben stehen in den nötigen Blöcken | T2 | Tokens und Kontrast (daedyx-ui) | offen |
| `tasks/AlarmFields.test.tsx` | listet die Erinnerungen mit lesbaren Texten, auch ungewöhnliche Werte | T3a | mehrere Erinnerungen | daedyx-tasks-ui `alarm_tests::zeigt_ungewoehnliche_werte_lesbar_und_ohne_doppelte` |
| `tasks/AlarmFields.test.tsx` | fügt über die Auswahl eine weitere Erinnerung hinzu (sortiert, ohne schon vorhandene) | T3a | mehrere Erinnerungen | daedyx-tasks-ui `alarm_tests::legt_zwei_an_behaelt_sie_beim_erneuten_oeffnen_und_entfernt_eine` |
| `tasks/AlarmFields.test.tsx` | entfernt eine einzelne Erinnerung | T3a | mehrere Erinnerungen | daedyx-tasks-ui `alarm_tests::legt_zwei_an_behaelt_sie_beim_erneuten_oeffnen_und_entfernt_eine` |
| `tasks/AlarmFields.test.tsx` | ohne Fälligkeit ist alles gesperrt und es steht ein Hinweis da | T3a | mehrere Erinnerungen | daedyx-tasks-ui `alarm_tests::ohne_faelligkeit_ist_alles_gesperrt_mit_hinweis` |
| `tasks/AlarmFields.test.tsx` | bietet bei zehn Erinnerungen nichts mehr zum Hinzufügen an | T3a | mehrere Erinnerungen | daedyx-tasks-ui `alarm_tests::zehn_gehen_die_elfte_wird_abgelehnt_und_nichts_mehr_angeboten` |
| `tasks/api.contract.test.ts` | ruft nur Befehle auf, die der Kern registriert hat, und umgekehrt | – | entfällt: Vertrag Oberfläche ↔ Tauri-Befehle (Plan 2.4) | offen |
| `tasks/api.contract.test.ts` | jeder Befehlsname der Befehlsdatei steht in der Befehlsliste | – | entfällt: Vertrag Oberfläche ↔ Tauri-Befehle (Plan 2.4) | offen |
| `tasks/boardModel.test.ts` | zeigt ohne Abschnitte nur „Ohne Abschnitt“, auch wenn sie leer ist | T3b | Board-Modell | daedyx-tasks-ui `model_tests::spalten_beginnen_mit_ohne_abschnitt_und_karten_sind_sortiert` |
| `tasks/boardModel.test.ts` | ordnet Karten nach Abschnitt und Handreihenfolge, Erledigte unten | T3b | Board-Modell | daedyx-tasks-ui `model_tests::karte_eines_unbekannten_abschnitts_steht_in_ohne_abschnitt_und_erledigte_unten` |
| `tasks/boardModel.test.ts` | blendet „Ohne Abschnitt“ erst ein, wenn dort Karten liegen | T3b | Board-Modell | – entfällt: Stil 1 zeigt „Ohne Abschnitt“ immer als erste Spalte (Desk M6a, Ablageziel für Karten) |
| `tasks/boardModel.test.ts` | legt Karten eines unbekannten (gelöschten) Abschnitts nach „Ohne Abschnitt“ | T3b | Board-Modell | daedyx-tasks-ui `model_tests::karte_eines_unbekannten_abschnitts_steht_in_ohne_abschnitt_und_erledigte_unten` |
| `tasks/boardModel.test.ts` | lässt Unteraufgaben aus den Spalten und zählt sie an der Oberaufgabe | T3b | Board-Modell | daedyx-tasks-ui `model_tests::spalten_beginnen_mit_ohne_abschnitt_und_karten_sind_sortiert`, `tree_model_tests::zaehler_ueber_drei_ebenen_zaehlt_alle_nachfahren` |
| `tasks/boardModel.test.ts` | setzt einen Abschnitt vor einen anderen | T3b | Board-Modell | daedyx-tasks-ui `model_tests::abschnitte_verschieben_und_ablegen` |
| `tasks/boardModel.test.ts` | ans Ende bei fehlendem Ziel und unverändert bei sich selbst | T3b | Board-Modell | offen |
| `tasks/boardModel.test.ts` | zählt an jeder Oberaufgabe alle Nachfahren | T3b | Board-Modell | daedyx-tasks-ui `tree_model_tests::zaehler_ueber_drei_ebenen_zaehlt_alle_nachfahren` |
| `tasks/boardModel.test.ts` | gibt die Ebene je Aufgabe an (0 = oberste); fehlende Oberaufgabe zählt als oberste | T3b | Board-Modell | daedyx-tasks-ui `tree_model_tests::handreihenfolge_ordnet_jede_ebene_direkt_unter_ihre_oberaufgabe`, `unteraufgabe_ohne_oberaufgabe_in_der_ansicht_zaehlt_als_oberste` |
| `tasks/dateEntry.test.ts` | Zahnarzt morgen 15 Uhr | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::tag_mit_uhrzeit_macht_eine_faelligkeit_in_utc_sekunden` |
| `tasks/dateEntry.test.ts` | wiederkehrend: Training jeden Montag 18:30 | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::wiederholungen` |
| `tasks/dateEntry.test.ts` | ganztägig mit Datum | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::ein_tag_macht_eine_ganztaegige_faelligkeit` |
| `tasks/dateEntry.test.ts` | Zeitraum von 14 bis 16 Uhr | T3a | Datumseingabe (Kern quick_when, T1a) | – entfällt: Termin-Angabe des Desk-Kalenders (die alte Datei war geteilt); Tasks nimmt Tag, Uhrzeit, Wiederholung und Priorität |
| `tasks/dateEntry.test.ts` | Ort nach 'bei' und 'in' | T3a | Datumseingabe (Kern quick_when, T1a) | – entfällt: Termin-Angabe des Desk-Kalenders (die alte Datei war geteilt); Tasks nimmt Tag, Uhrzeit, Wiederholung und Priorität |
| `tasks/dateEntry.test.ts` | Monatsname, Uhrzeit und Dauer | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::datum_mit_monatsnamen_und_jahr` (Dauer: Termin-Angabe des Desk-Kalenders (die alte Datei war geteilt); Tasks nimmt Tag, Uhrzeit, Wiederholung und Priorität) |
| `tasks/dateEntry.test.ts` | Wochentage: der nächste, nie heute | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::wochentage_meinen_das_naechste_mal_nie_heute` |
| `tasks/dateEntry.test.ts` | Datum ohne Jahr, das schon vorbei ist, gilt fürs nächste Jahr | T3a | Datumseingabe (Kern quick_when, T1a) | offen |
| `tasks/dateEntry.test.ts` | Wiederholungen: täglich, wöchentlich, monatlich, jährlich | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::wiederholungen` |
| `tasks/dateEntry.test.ts` | nur ein Titel wird ein ganztägiger Termin heute; Ganztägig lässt die Uhrzeit weg | T3a | Datumseingabe (Kern quick_when, T1a) | – entfällt: Termin-Angabe des Desk-Kalenders (die alte Datei war geteilt); Tasks nimmt Tag, Uhrzeit, Wiederholung und Priorität; in Tasks bleibt ein Titel ein Titel: daedyx-tasks-core `quick_tests::nur_ein_titel_bleibt_ein_titel` |
| `tasks/dateEntry.test.ts` | merkt, ob der Text einen Zeitpunkt nannte | T3a | Datumseingabe (Kern quick_when, T1a) | offen |
| `tasks/dateEntry.test.ts` | liefert null bei leerer Eingabe, fehlendem Titel und ungültigen Daten | T3a | Datumseingabe (Kern quick_when, T1a) | daedyx-tasks-core `quick_tests::leere_eingabe_ergibt_nichts`, `nur_ein_datum_ohne_titel_ergibt_nichts`, `unmoegliche_angaben_bleiben_text` |
| `tasks/dueDay.test.ts` | ohne Fälligkeit gibt es keinen Tag | T3a | Fälligkeit (Kern due_time) | offen |
| `tasks/dueDay.test.ts` | ein Datum bleibt, ohne Uhrzeit | T3a | Fälligkeit (Kern due_time) | offen |
| `tasks/dueDay.test.ts` | Tag und Uhrzeit lassen sich in Ortszeit hin und zurück rechnen | T3a | Fälligkeit (Kern due_time) | offen |
| `tasks/dueDay.test.ts` | leerer Tag löscht, Tag ohne Uhrzeit ist ganztägig | T3a | Fälligkeit (Kern due_time) | offen |
| `tasks/dueDay.test.ts` | unlesbare Sekunden ergeben keinen Tag | T3a | Fälligkeit (Kern due_time) | offen |
| `tasks/dueLabel.test.ts` | ohne Fälligkeit nichts | T3a | Fälligkeitsmarke | offen |
| `tasks/dueLabel.test.ts` | heute, morgen, gestern | T3a | Fälligkeitsmarke | offen |
| `tasks/dueLabel.test.ts` | andere Tage mit Wochentag, in fremden Jahren mit Jahr | T3a | Fälligkeitsmarke | offen |
| `tasks/dueLabel.test.ts` | mit Uhrzeit hängt sie dran | T3a | Fälligkeitsmarke | offen |
| `tasks/dueLabel.test.ts` | erledigte Aufgaben sind nie überfällig | T3a | Fälligkeitsmarke | offen |
| `tasks/quickEntry.test.ts` | nur ein Titel bleibt ein Titel | T3a | Schnelleingabe (Kern quick, T1a) | daedyx-tasks-core `quick_tests::nur_ein_titel_bleibt_ein_titel` |
| `tasks/quickEntry.test.ts` | leere Eingabe ergibt nichts | T3a | Schnelleingabe (Kern quick, T1a) | daedyx-tasks-core `quick_tests::leere_eingabe_ergibt_nichts` |
| `tasks/quickEntry.test.ts` | Ausrufezeichen setzen die Priorität und gehören nicht zum Titel | T3a | Schnelleingabe (Kern quick, T1a) | daedyx-tasks-core `quick_tests::ausrufezeichen_setzen_die_prioritaet` |
| `tasks/quickEntry.test.ts` | ein Tag macht eine ganztägige Fälligkeit | T3a | Schnelleingabe (Kern quick, T1a) | daedyx-tasks-core `quick_tests::ein_tag_macht_eine_ganztaegige_faelligkeit` |
| `tasks/quickEntry.test.ts` | Tag mit Uhrzeit macht eine Fälligkeit mit Uhrzeit (UTC-Sekunden) | T3a | Schnelleingabe (Kern quick, T1a) | daedyx-tasks-core `quick_tests::tag_mit_uhrzeit_macht_eine_faelligkeit_in_utc_sekunden` |
| `tasks/RepeatFields.test.tsx` | monatlich bietet „Fester Tag im Monat“, leer = Tag der Fälligkeit | T3a | Wiederholung voll | daedyx-tasks-ui `repeat_tests::fester_monatstag_begrenzt_auf_1_bis_31_und_leer_loescht_den_anker` |
| `tasks/RepeatFields.test.tsx` | jeden 15. des Monats eintragen setzt monthDay | T3a | Wiederholung voll | daedyx-tasks-ui `repeat_tests::fester_monatstag_begrenzt_auf_1_bis_31_und_leer_loescht_den_anker` |
| `tasks/RepeatFields.test.tsx` | begrenzt auf 1 bis 31 und löscht den Anker bei leerem Feld | T3a | Wiederholung voll | daedyx-tasks-ui `repeat_tests::fester_monatstag_begrenzt_auf_1_bis_31_und_leer_loescht_den_anker` |
| `tasks/RepeatFields.test.tsx` | gibt es für täglich und wöchentlich nicht; Wechsel zu täglich entfernt den Anker | T3a | Wiederholung voll | daedyx-tasks-ui `repeat_tests::abstand_und_ab_abhaken`, `wechsel_der_art_und_begrenzen_der_zahlen` |
| `tasks/RepeatFields.test.tsx` | jährlich zeigt den Tag ebenfalls | T3a | Wiederholung voll | daedyx-tasks-ui `repeat_tests::wechsel_der_art_und_begrenzen_der_zahlen` |
| `tasks/repeatNext.test.ts` | monatlich ohne festen Tag nimmt den Tag der Fälligkeit, kurze Monate den letzten Tag | T3a | Kern repeat (Fake in TS entfällt) | daedyx-tasks-core `repeat_tests::monthly_clamps_to_month_end_and_yearly_handles_leap_day`, `without_an_anchor_the_day_drifts_down_and_stays_there` |
| `tasks/repeatNext.test.ts` | monatlich mit festem Tag springt auf diesen Tag, auch wenn die Fälligkeit anders lag | T3a | Kern repeat (Fake in TS entfällt) | daedyx-tasks-core `repeat_tests::the_anchor_wins_over_the_day_of_the_base_date_and_works_with_intervals_and_leap_years` |
| `tasks/repeatNext.test.ts` | der Anker bleibt nach einem kurzen Monat erhalten (31. → 28. Feb. → 31. März) | T3a | Kern repeat (Fake in TS entfällt) | daedyx-tasks-ui `repeat_tests::monatstag_31_rutscht_in_kurzen_monaten_und_kehrt_zurueck` (über den Kern) |
| `tasks/repeatNext.test.ts` | beachtet den Abstand in Monaten und das Jahr | T3a | Kern repeat (Fake in TS entfällt) | daedyx-tasks-core `repeat_tests::the_anchor_wins_over_the_day_of_the_base_date_and_works_with_intervals_and_leap_years` |
| `tasks/repeatNext.test.ts` | täglich und wöchentlich bleiben wie bisher | T3a | Kern repeat (Fake in TS entfällt) | daedyx-tasks-core `repeat_tests::daily_and_interval`, `weekly_plain_and_with_weekdays` |
| `tasks/sorting.test.ts` | Handreihenfolge nach Position, Unteraufgaben direkt nach der Oberaufgabe | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::handreihenfolge_ordnet_jede_ebene_direkt_unter_ihre_oberaufgabe` |
| `tasks/sorting.test.ts` | nach Fälligkeit, ohne Datum zuletzt | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::nach_faelligkeit_ohne_datum_zuletzt` |
| `tasks/sorting.test.ts` | nach Priorität, hohe zuerst | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::nach_prioritaet_hohe_zuerst` |
| `tasks/sorting.test.ts` | nach Titel ohne Beachtung von Groß- und Kleinschreibung und mit Umlauten | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::nach_titel_ohne_gross_klein_und_mit_umlauten` |
| `tasks/sorting.test.ts` | Unteraufgabe ohne Oberaufgabe in der Ansicht zählt als oberste | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::unteraufgabe_ohne_oberaufgabe_in_der_ansicht_zaehlt_als_oberste` |
| `tasks/sorting.test.ts` | Position zwischen Nachbarn und am Rand | T3a | vier Sortierungen | daedyx-tasks-ui `model_tests::einfuegeposition_liegt_zwischen_den_nachbarn` |
| `tasks/sorting.test.ts` | Position vor einer Aufgabe, am Anfang und am Ende | T3a | vier Sortierungen | daedyx-tasks-ui `sort_drag_tests::zeile_mit_der_maus_vor_eine_andere_ziehen`, `alt_pfeil_verschiebt_ohne_maus_und_nur_von_hand` |
| `tasks/sorting.test.ts` | ordnet jede Ebene in Handreihenfolge direkt unter ihre Oberaufgabe | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::handreihenfolge_ordnet_jede_ebene_direkt_unter_ihre_oberaufgabe` |
| `tasks/sorting.test.ts` | sortiert nur die obersten Aufgaben nach Titel, der Unterbaum folgt unverändert | T3a | vier Sortierungen | daedyx-tasks-ui `tree_model_tests::sortiert_nur_die_obersten_der_unterbaum_folgt_unveraendert` |
| `tasks/TaskBoard.test.tsx` | schaltet zwischen Liste und Board um und merkt die Wahl je Liste | T3b | Board | daedyx-tasks-ui `board_more_tests::board_modus_wird_je_liste_gemerkt` (für die Sitzung) |
| `tasks/TaskBoard.test.tsx` | legt Abschnitte und Karten in Abschnitten an | T3b | Board | daedyx-tasks-ui `parity_tests::abschnitte_anlegen_umbenennen_ordnen_und_loeschen_die_karten_bleiben`, `board_more_tests::plus_karte_legt_die_karte_in_ihrer_spalte_an_und_bleibt_offen` |
| `tasks/TaskBoard.test.tsx` | verschiebt Karten per Ziehen in einen Abschnitt und innerhalb der Spalte | T3b | Board | daedyx-tasks-ui `parity_tests::karte_per_ziehen_in_einen_abschnitt_und_abgehakt_unten_im_abschnitt`; innerhalb der Spalte Desk `tasks_board_tests::alt_pfeil_verschiebt_karten_zwischen_und_in_spalten` |
| `tasks/TaskBoard.test.tsx` | lässt eine abgehakte Karte im Abschnitt, durchgestrichen und unten | T3b | Board | daedyx-tasks-ui `parity_tests::karte_per_ziehen_in_einen_abschnitt_und_abgehakt_unten_im_abschnitt` |
| `tasks/TaskBoard.test.tsx` | benennt einen Abschnitt per Doppelklick um und löscht ihn mit Rückfrage, die Karten bleiben | T3b | Board | daedyx-tasks-ui `parity_tests::abschnitte_anlegen_umbenennen_ordnen_und_loeschen_die_karten_bleiben` (Umbenennen mit F2 statt Doppelklick: Desk `tasks_board_tests::abschnitt_umbenennen_mit_f2_und_verschieben_mit_alt_pfeil`) |
| `tasks/TaskBoard.test.tsx` | sortiert Abschnitte per Ziehen am Kopf | T3b | Board | daedyx-tasks-ui `parity_tests::abschnitte_anlegen_umbenennen_ordnen_und_loeschen_die_karten_bleiben` |
| `tasks/TaskBoard.test.tsx` | wählt den Abschnitt im Detailbereich | T3b | Board | daedyx-tasks-ui `sort_drag_tests::abschnitt_im_detail_und_zustaendig_als_platzhalter` |
| `tasks/TaskBoard.test.tsx` | trägt beliebig viele Abschnitte, behält ihre Reihenfolge und nimmt Karten im letzten auf | T3b | Board | daedyx-tasks-ui `parity_tests::traegt_viele_abschnitte_in_ihrer_reihenfolge_und_nimmt_karten_im_letzten_auf` |
| `tasks/taskNotify.test.ts` | nennt, wann die Aufgabe fällig ist | T4 | Benachrichtigungstexte (T-F6, neutral) | offen |
| `tasks/taskNotify.test.ts` | zeigt außerhalb der App nichts und wirft nicht | T4 | Benachrichtigungstexte (T-F6, neutral) | offen |
| `tasks/TasksArea.test.tsx` | zeigt Smart-Listen mit Zahlen und startet bei „Heute“ | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::smart_listen_mit_zahlen_und_start_bei_heute` |
| `tasks/TasksArea.test.tsx` | zeigt eine Liste mit eingerückten Unteraufgaben | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `deep_tests::eingerueckt_nach_ebene_mit_ebene_fuer_screenreader_und_zaehler_ueber_alle` |
| `tasks/TasksArea.test.tsx` | legt per Schnelleingabe eine Aufgabe mit Priorität an | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::schnelleingabe_mit_prioritaet_und_in_heute_mit_faelligkeit_heute` |
| `tasks/TasksArea.test.tsx` | legt in „Heute“ Aufgaben mit Fälligkeit heute an | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::schnelleingabe_mit_prioritaet_und_in_heute_mit_faelligkeit_heute` |
| `tasks/TasksArea.test.tsx` | hakt eine Aufgabe ab und wieder auf | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::hakt_eine_aufgabe_ab_und_wieder_auf` |
| `tasks/TasksArea.test.tsx` | öffnet Einzelheiten und speichert Titel und Priorität | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::oeffnet_einzelheiten_und_speichert_titel_und_prioritaet` |
| `tasks/TasksArea.test.tsx` | legt eine Liste an, benennt sie um und löscht sie nach Rückfrage | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `parity_tests::legt_eine_liste_an_benennt_sie_um_und_loescht_sie_nach_rueckfrage` |
| `tasks/TasksArea.test.tsx` | zeigt Suchtreffer, wenn die Schnellsuche sie anfordert, und beendet die Suche | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `board_more_tests::suchfeld_zeigt_treffer_und_leeren_beendet_die_suche` (Suchfeld statt Schnellsuche, T-F4) |
| `tasks/TasksArea.test.tsx` | legt Unteraufgaben im Detailbereich an | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `deep_tests::unteraufgabe_im_detail_anlegen_und_oeffnen` |
| `tasks/TasksArea.test.tsx` | sortiert per Ziehen um und legt Aufgaben per Ziehen in eine andere Liste | T3a/T3b | Liste, Leiste, Suche (T-F4) | daedyx-tasks-ui `sort_drag_tests::zeile_mit_der_maus_vor_eine_andere_ziehen`, `lists_tests::aufgabe_aus_der_liste_auf_eine_andere_liste_ziehen` |
| `tasks/TasksDeep.test.tsx` | legt zwei Erinnerungen an, behält sie beim erneuten Öffnen und entfernt eine | T3a | Unteraufgaben bis Ebene 6 | daedyx-tasks-ui `alarm_tests::legt_zwei_an_behaelt_sie_beim_erneuten_oeffnen_und_entfernt_eine` |
| `tasks/TasksDeep.test.tsx` | Fälligkeit entfernen löscht auch die Erinnerungen | T3a | Unteraufgaben bis Ebene 6 | daedyx-tasks-ui `alarm_tests::faelligkeit_entfernen_loescht_auch_die_erinnerungen` |
| `tasks/TasksDeep.test.tsx` | legt eine Unteraufgabe unter einer Unteraufgabe an, eingerückt nach Ebene, Zähler an der Oberaufgabe zählt alle | T3a | Unteraufgaben bis Ebene 6 | daedyx-tasks-ui `deep_tests::eingerueckt_nach_ebene_mit_ebene_fuer_screenreader_und_zaehler_ueber_alle`, `tree_model_tests::zaehler_ueber_drei_ebenen_zaehlt_alle_nachfahren` |
| `tasks/TasksDeep.test.tsx` | auf der letzten Ebene gibt es keine weitere Unteraufgabe | T3a | Unteraufgaben bis Ebene 6 | daedyx-tasks-ui `deep_tests::ebene_6_geht_ebene_7_ergibt_hinweis_statt_absturz` |
| `tasks/TasksDeep.test.tsx` | zeigt in der Unteraufgabe die Oberaufgabe und springt dorthin zurück | T3a | Unteraufgaben bis Ebene 6 | daedyx-tasks-ui `deep_tests::zeigt_in_der_unteraufgabe_die_oberaufgabe_und_springt_dorthin_zurueck` |
| `tasks/useDebouncedField.test.tsx` | speichert nach der Pause nur den letzten Stand, nicht jeden Tastendruck | T3a | Felder gelten beim Verlassen | – entfällt: kein Zeitversatz in egui; Ersatz `board_more_tests::feld_speichert_mit_enter_und_beim_verlassen` |
| `tasks/useDebouncedField.test.tsx` | speichert sofort beim Verlassen des Feldes, und nur einmal | T3a | Felder gelten beim Verlassen | daedyx-tasks-ui `board_more_tests::feld_speichert_mit_enter_und_beim_verlassen` |
| `tasks/useDebouncedField.test.tsx` | übernimmt einen neuen Wert von außen, wenn nichts getippt wird | T3a | Felder gelten beim Verlassen | daedyx-tasks-ui `repeat_tests::monatstag_31_rutscht_in_kurzen_monaten_und_kehrt_zurueck` (Detail lädt neu, wenn der Kern die Aufgabe ändert: `detail::sync`) |
| `tasks/useDebouncedField.test.tsx` | speichert Ungespeichertes beim Schließen des Bereichs | T3a | Felder gelten beim Verlassen | daedyx-tasks-ui `board_more_tests::kein_verlust_beim_wechsel_der_aufgabe` |
| `theme/darkStyle.test.ts` | lässt die Mitte unverändert | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | mischt links Schwarz bei (bis 80 %), rechts Weiß (bis 20 %) | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | begrenzt, rundet und ergänzt fehlende Felder mit dem Standard | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | rechnet die Stufen in Variablen um (Tönung 30 = Faktor 1) | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | setzt die Variablen auf dem Element | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | liefert den Standard bei fehlendem oder kaputtem Wert | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | übernimmt gültige Werte und begrenzt ungültige | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | enthalten Neutralgrau und den heutigen Standard | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/darkStyle.test.ts` | liefern ein hsl-Muster | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/useDarkStyle.test.ts` | startet mit dem Standard und setzt die Variablen | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/useDarkStyle.test.ts` | ändert einzelne Felder, setzt die Variablen auf <html> und merkt sich die Wahl | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/useDarkStyle.test.ts` | liest die gespeicherte Wahl beim Start | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/useDarkStyle.test.ts` | setzt mit reset zurück | T4 | Dunkelmodus-Feinstellung (T-F3) | offen |
| `theme/useTheme.test.ts` | setzt data-theme bei hell und dunkel | T2 | Hell/Dunkel | offen |
| `theme/useTheme.test.ts` | entfernt data-theme bei system | T2 | Hell/Dunkel | offen |
| `theme/useTheme.test.ts` | startet mit system | T2 | Hell/Dunkel | offen |
| `theme/useTheme.test.ts` | wechselt system -> hell -> dunkel -> system und merkt sich die Wahl | T2 | Hell/Dunkel | offen |
| `theme/useTheme.test.ts` | liest die gespeicherte Wahl beim Start | T2 | Hell/Dunkel | offen |
| `ui/buttons.test.ts` | gesperrt: lesbare Schrift aus dem Token, kein Ausgrauen über opacity | T2 | Knöpfe aus daedyx-ui | offen |
| `ui/buttons.test.ts` | gesperrte Hauptknöpfe bekommen eine ruhige Fläche statt Akzent mit Schleier | T2 | Knöpfe aus daedyx-ui | offen |
| `ui/buttons.test.ts` | arbeitend (aria-busy): Spinner, Fortschrittszeiger, Hauptknöpfe leicht entsättigt, Schrift unverändert | T2 | Knöpfe aus daedyx-ui | offen |
| `ui/buttons.test.ts` | Spinner steht still bei reduzierter Bewegung | T2 | Knöpfe aus daedyx-ui | offen |
| `ui/buttons.test.ts` | kein anderes Stylesheet graut gesperrte Knöpfe über opacity aus | T2 | Knöpfe aus daedyx-ui | offen |
| `ui/ContextMenu.test.tsx` | ist ein Menü mit Einträgen und fokussiert den ersten | T3b | Menüs (daedyx-ui menu_button) | daedyx-ui `widgets::menu_tests::oeffnen_fokussiert_den_ersten_eintrag_und_pfeile_wandern_im_kreis` |
| `ui/ContextMenu.test.tsx` | wandert mit Pfeiltasten im Kreis und schließt mit Escape (mit Fokusrückgabe) | T3b | Menüs (daedyx-ui menu_button) | daedyx-ui `widgets::menu_tests::oeffnen_fokussiert_den_ersten_eintrag_und_pfeile_wandern_im_kreis`, `escape_schliesst_und_gibt_den_fokus_an_den_knopf` |
| `ui/ContextMenu.test.tsx` | führt einen Eintrag aus und schließt | T3b | Menüs (daedyx-ui menu_button) | daedyx-ui `widgets::menu_tests::enter_waehlt_und_schliesst` |
| `ui/ContextMenu.test.tsx` | lässt deaktivierte Einträge erreichbar, aber wirkungslos | T3b | Menüs (daedyx-ui menu_button) | – entfällt: `menu_button` kennt keine deaktivierten Einträge (kein Menü in Tasks braucht sie) |
| `ui/ContextMenu.test.tsx` | schließt bei einem Klick daneben | T3b | Menüs (daedyx-ui menu_button) | daedyx-ui `widgets::menu_tests::klick_daneben_schliesst_ohne_auswahl` |
| `ui/ResizeHandle.test.tsx` | ist ein senkrechter, fokussierbarer Trenner mit Werten | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/ResizeHandle.test.tsx` | ändert die Breite mit den Pfeiltasten (Umschalt = große Schritte) und springt mit Home/End | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/ResizeHandle.test.tsx` | begrenzt die Tastenschritte auf Mindest- und Höchstbreite | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/ResizeHandle.test.tsx` | zieht mit der Maus relativ zum Startpunkt und hört nach dem Loslassen auf | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/ResizeHandle.test.tsx` | zieht nur mit der linken Maustaste | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/ResizeHandle.test.tsx` | stellt mit Doppelklick die Standardbreite wieder her | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/usePaneWidth.test.ts` | startet mit der Standardbreite | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/usePaneWidth.test.ts` | begrenzt, rundet und speichert die Breite | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/usePaneWidth.test.ts` | liest die gespeicherte Breite beim Start, begrenzt und ignoriert Unsinn | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `ui/usePaneWidth.test.ts` | stellt mit reset die Standardbreite wieder her | – | entfällt: feste Breiten im Stil 1 (T-Rest) | offen |
| `updates/autoCheck.test.ts` | sucht, wenn nie gesucht wurde | T4 | Updates (U-F2) | offen |
| `updates/autoCheck.test.ts` | sucht höchstens einmal pro Tag | T4 | Updates (U-F2) | offen |
| `updates/autoCheck.test.ts` | sucht nie, wenn die Einstellung aus ist | T4 | Updates (U-F2) | offen |
| `updates/autoCheck.test.ts` | ein Zeitpunkt in der Zukunft (Uhr verstellt) zählt als nie gesucht | T4 | Updates (U-F2) | offen |
| `updates/useStartupCheck.test.ts` | liefert die gefundene Version und merkt sich den Zeitpunkt | T4 | Updates (U-F2) | offen |
| `updates/useStartupCheck.test.ts` | sucht am selben Tag nicht noch einmal | T4 | Updates (U-F2) | offen |
| `updates/useStartupCheck.test.ts` | sucht nicht, wenn die Einstellung aus ist | T4 | Updates (U-F2) | offen |
| `updates/useStartupCheck.test.ts` | ein Fehler bleibt stumm | T4 | Updates (U-F2) | offen |
| `updates/useUpdater.test.ts` | liest die aktuelle Version | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | ohne neue Version: aktuell | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | mit neuer Version: verfügbar mit Info | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | Fehler beim Suchen werden gemeldet, nie geworfen | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | ohne eingetragenen Update-Schlüssel: ehrlich "nicht eingerichtet" statt Fehlertext des Plugins | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | Installieren meldet den Fortschritt und endet bereit | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | Die Rückfrage nach ungespeicherten Entwürfen kann das Installieren anhalten | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | Neustart nur im Zustand bereit | T4 | Updates (U-F1) | offen |
| `updates/useUpdater.test.ts` | Fehler beim Installieren führen in den Zustand Fehler | T4 | Updates (U-F1) | offen |
