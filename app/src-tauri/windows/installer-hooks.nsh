; Schreibt nach jeder Installation (auch nach Selbst-Updates) die app.json, aus der der Daedyx Launcher
; die installierte Version und das Startprogramm liest. Der Setup-Lauf des Updaters ist derselbe Lauf.
!macro NSIS_HOOK_POSTINSTALL
  FileOpen $0 "$INSTDIR\app.json" w
  FileWrite $0 '{"version": "${VERSION}", "exe": "${MAINBINARYNAME}.exe"}'
  FileClose $0
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$INSTDIR\app.json"
!macroend
