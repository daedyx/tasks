; Setup der Produkt-Fassung von Daedyx Tasks (T7, Lücke L2). Hier stehen nur die Kennungen; den Ablauf (Launcher,
; Update aus der App, Deinstallation) liefert die Vorlage aus daedyx-build, dieselbe wie für die Vorschau.
; Bauen: im Release-Lauf (.github/workflows/release.yml, Schritt "Setup bauen") über installer-bauen.ps1 der Vorlage.
; Kennungen wie in crates/tasks-app/src/ids.rs mit Merkmal produkt: Produkt „Daedyx Tasks“, App-Kennung
; de.daedyx.tasks, Programm daedyx-tasks.exe (Programmziel nur mit produkt). Daraus folgen der Uninstall-Schlüssel
; HKCU\...\Uninstall\Daedyx Tasks, den der Launcher sucht (Review U3, F-U3-5), und der Ordner-Merkschlüssel
; HKCU\Software\daedyx\Daedyx Tasks. Test: crates/tasks-app/src/setup_produkt_tests.rs.

; Lizenzhinweise der Bibliotheken (Urheber und Lizenztexte); erzeugt von scripts/vorschau-lizenzen.mjs.
!ifndef LICENSES
  !define LICENSES "${__FILEDIR__}\..\app\src-tauri\crates\tasks-app\lizenztexte.txt"
!endif
!define PRODUCT "Daedyx Tasks"
!define AUMID "de.daedyx.tasks"
!define EXE "daedyx-tasks.exe"

!include "${VORLAGE}\app-vorschau.nsi"
