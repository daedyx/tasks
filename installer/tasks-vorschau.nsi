; Setup der egui-Vorschau von Daedyx Tasks (Tasks T6). Hier stehen nur die Kennungen; den Ablauf (Launcher,
; Tauri-Updater, Deinstallation) liefert die Vorlage aus daedyx-build (Tasks T1b, aus Desk M8).
; Bauen: scripts/installer-vorschau.ps1 (sucht die Vorlage über cargo metadata, ruft makensis mit
; /DVERSION /DSRCDIR /DICON /DOUT /DVORLAGE). Probe: scripts/installer-probe-vorschau.ps1.
; Nur Kennungen der VORSCHAU, dieselben wie in crates/tasks-app/src/ids.rs: Produkt „Daedyx Tasks Vorschau“,
; App-Kennung de.daedyx.tasks.vorschau, Programm daedyx-tasks-vorschau.exe. Ordner-Merkschlüssel daraus:
; HKCU\Software\daedyx\Daedyx Tasks Vorschau. Die echte Installation „Daedyx Tasks“ wird nie berührt.

; Lizenzhinweise der Bibliotheken (Urheber und Lizenztexte); erzeugt von scripts/vorschau-lizenzen.mjs.
!ifndef LICENSES
  !define LICENSES "${__FILEDIR__}\..\app\src-tauri\crates\tasks-app\lizenztexte.txt"
!endif
; Eigener Name, damit die echte Installation „Daedyx Tasks“ unberührt bleibt (Produkt ab T7: "Daedyx Tasks").
!define PRODUCT "Daedyx Tasks Vorschau"
!define AUMID "de.daedyx.tasks.vorschau"
!define EXE "daedyx-tasks-vorschau.exe"

!include "${VORLAGE}\app-vorschau.nsi"
