# Abnahme K4 „Tasks-Funktionen“

Stand: 2026-11-19, Zweig `k4-funktionen`. Teil 1 (A–E): Board mit vielen Abschnitten, Hintergrundlauf. Teil 2 (F–H): mehrere Erinnerungen, fester Monatstag, Unteraufgaben über mehrere Ebenen. Teil 2 braucht den Kern Stand 3 (`rev 86ea2fc`, Zweig `k2-schema-3`, noch ohne Tag).

## Vorab (bitte zuerst)
1. **Handkopie des Datenordners:** Ordner `%APPDATA%\de.daedyx.tasks.dev` (Entwicklungs-Build) bzw. `%APPDATA%\de.daedyx.tasks` (installiert) im Explorer kopieren, z. B. auf den Desktop. Nur falls dort schon eigene Testlisten liegen. Es dürfen nur erfundene Testdaten drin sein. **Wichtig für Teil 2:** Beim ersten Start hebt der Kern die Datenbank von Stand 2 auf Stand 3 (neue Tabelle für Erinnerungen). Das ändert die Datei dauerhaft; ein Zurück gibt es nur über die Handkopie. Eine automatische Sicherung vor dem Umbau gibt es in Tasks noch nicht.
2. App starten: `cd D:\Programmierung\daedyx-tasks-k4\app` und `npm run tauri:dev` (oder gebauten Installer nehmen).

## A. Board mit vielen Abschnitten
- [ ] Liste „Test“ anlegen, auf **Board** umschalten.
- [ ] Zehn Abschnitte nacheinander über „+ Abschnitt“ anlegen (Eins bis Zehn). Erwartung: alle erscheinen in der Reihenfolge nebeneinander, das Board lässt sich seitwärts scrollen, nichts wird abgeschnitten oder gestapelt.
- [ ] Eine Karte in den letzten Abschnitt ziehen. Erwartung: sie bleibt dort, auch nach App-Neustart.

## B. Im Infobereich weiterlaufen
- [ ] Einstellungen öffnen (Zahnrad). Erwartung: Abschnitt „Hintergrund und Erinnerungen“ mit zwei Schaltern; „Im Infobereich weiterlaufen“ ist **an**.
- [ ] Unten rechts neben der Uhr (ggf. über den Pfeil „Ausgeblendete Symbole“): Symbol von Daedyx Tasks sichtbar. Rechtsklick zeigt „Daedyx Tasks öffnen“ und „Beenden“.
- [ ] Fenster mit dem X schließen. Erwartung: Fenster verschwindet, **Programm läuft weiter** (Symbol bleibt, Task-Manager zeigt `daedyx-tasks`).
- [ ] Linksklick auf das Symbol oder „Daedyx Tasks öffnen“: Fenster kommt zurück, Inhalt unverändert.
- [ ] Rechtsklick → „Beenden“: Programm ist wirklich weg (Symbol verschwindet, Prozess weg).
- [ ] Schalter „Im Infobereich weiterlaufen“ ausschalten, Fenster schließen: Programm beendet sich dann ganz.

## C. Erinnerung bei geschlossenem Fenster
- [ ] Aufgabe „Test Erinnerung“ mit Fälligkeit **in 3 Minuten** (Datum + Uhrzeit) und Erinnerung „zur Fälligkeit“ anlegen.
- [ ] Fenster schließen (Schalter B an). Erwartung: zur Zeit erscheint die Windows-Benachrichtigung „Test Erinnerung“, ohne dass das Fenster offen ist. (Windows fragt beim ersten Mal ggf. nach der Erlaubnis für Benachrichtigungen; bitte erlauben.)

## D. Einzelinstanz
- [ ] Fenster verstecken (X). Daedyx Tasks erneut starten (Startmenü oder `npm run tauri:dev` nicht, sondern installierte Version/Exe). Erwartung: es öffnet sich **kein zweites** Fenster, sondern das versteckte kommt nach vorn.

## E. Mit Windows starten
- [ ] Schalter „Mit Windows starten“ einschalten. Erwartung: kein Fehlertext. Windows-Einstellungen → Apps → Autostart zeigt „Daedyx Tasks“.
- [ ] Windows abmelden und wieder anmelden. Erwartung: Daedyx Tasks läuft im Infobereich, **ohne** dass sich ein Fenster öffnet; Klick auf das Symbol zeigt es.
- [ ] Schalter wieder ausschalten (Eintrag im Autostart verschwindet).

## F. Mehrere Erinnerungen
- [ ] Aufgabe „Test Mehrfach“ mit Fälligkeit **heute in 10 Minuten** (Datum und Uhrzeit) anlegen, Detailbereich öffnen. Erwartung: Abschnitt „Erinnerungen“, Auswahl „Erinnerung hinzufügen …“.
- [ ] „5 Minuten vorher“ und „Zur Fälligkeit“ hinzufügen. Erwartung: beide stehen als Liste, jeweils mit „Entfernen“.
- [ ] Fenster schließen (Hintergrundlauf an). Erwartung: **zweimal** eine Benachrichtigung, nach 5 Minuten „Fällig in 5 Minuten“ und zur Fälligkeit „Jetzt fällig“.
- [ ] Eine Erinnerung entfernen, App beenden und neu starten. Erwartung: die andere ist noch da.
- [ ] Fälligkeit der Aufgabe löschen. Erwartung: die Erinnerungen verschwinden, die Auswahl ist gesperrt mit Hinweis.

## G. Fester Monatstag
- [ ] Aufgabe „Test 15.“ mit Fälligkeit am 1. des nächsten Monats, Wiederholen „Monatlich“. Erwartung: Feld „Fester Tag im Monat (1–31)“.
- [ ] 15 eintragen, Aufgabe abhaken. Erwartung: neue Fälligkeit ist der **15.** des Folgemonats (nicht der 1.).
- [ ] Aufgabe „Test 31.“ mit Fälligkeit 31. Januar (oder dem nächsten 31.), „Monatlich“, fester Tag 31. Erwartung: nach dem Abhaken der letzte Tag des kurzen Monats (28./29./30.), danach wieder der 31.
- [ ] Feld leeren: Verhalten wie bisher (Tag der Fälligkeit).

## H. Unteraufgaben über mehrere Ebenen
- [ ] Aufgabe öffnen, Unteraufgabe „Ebene 1“ anlegen, dann auf „Öffnen“ bei der Unteraufgabe, dort „Ebene 2“ anlegen und so weiter. Erwartung: jede Ebene ist in der Liste weiter eingerückt.
- [ ] In einer Unteraufgabe „↑ Oberaufgabe“ anklicken. Erwartung: Sprung zur Oberaufgabe.
- [ ] Auf Ebene 6 weitermachen. Erwartung: kein Eingabefeld mehr, Hinweis „Auf dieser Ebene sind keine weiteren Unteraufgaben möglich.“
- [ ] Die oberste Aufgabe zeigt „x von y Unteraufgaben“ mit **allen** Ebenen; im Board erscheinen nur oberste Aufgaben als Karten.
- [ ] Oberste Aufgabe löschen. Erwartung: alle Unteraufgaben sind mit weg.

## Nach der Abnahme
Testdaten bzw. kopierten Datenordner bei Bedarf wieder zurückspielen oder löschen.
