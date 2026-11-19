# Abnahme K4 „Tasks-Funktionen“ (Teil 1: ohne Kernänderung)

Stand: 2026-11-19, Zweig `k4-funktionen`. Gebaut ist, was ohne Änderung am Kern geht: Board mit beliebig vielen Abschnitten (belegt), Hintergrundlauf mit Symbol im Infobereich, Autostart und Einzelinstanz. **Nicht gebaut** (braucht den Kern, siehe `docs/STAND.md`): Unteraufgaben über mehrere Ebenen, Monatstag-Anker, mehrere Erinnerungen.

## Vorab (bitte zuerst)
1. **Handkopie des Datenordners:** Ordner `%APPDATA%\de.daedyx.tasks.dev` (Entwicklungs-Build) bzw. `%APPDATA%\de.daedyx.tasks` (installiert) im Explorer kopieren, z. B. auf den Desktop. Nur falls dort schon eigene Testlisten liegen. Es dürfen nur erfundene Testdaten drin sein.
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
- [ ] Schalter „Mit Windows starten“ einschalten. Erwartung: kein Fehlertext. Windows-Einstellungen → Apps → Autostart zeigt „daedyx-tasks“.
- [ ] Windows abmelden und wieder anmelden. Erwartung: Daedyx Tasks läuft im Infobereich, **ohne** dass sich ein Fenster öffnet; Klick auf das Symbol zeigt es.
- [ ] Schalter wieder ausschalten (Eintrag im Autostart verschwindet).

## Nach der Abnahme
Testdaten bzw. kopierten Datenordner bei Bedarf wieder zurückspielen oder löschen.
