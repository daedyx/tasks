# Symbole der Daedyx-Apps

Eigene Zeichnung, entstanden aus Code (keine Bildgeneratoren, keine fremden Vorlagen, keine Piktogramm-Sammlung). Richtung „Fläche“ (Björn, 2026-11-19): weiße Flächen auf Indigo, Bernstein als Akzent, Kachel mit Rundung 14 von 72 wie die Launcher-Marke („Labyrinth-D“, `daedyx-launcher`). Urheberschaft: Daedyx contributors, Lizenz wie das Projekt (MIT).

| App | Motiv |
|---|---|
| Desk | Postfach mit Kalenderringen |
| Tasks | abgehakte Liste |
| Board | Zeichenfläche mit zwei verbundenen Karten (Name „Board“ ist ein Arbeitsname; das Repository fehlt noch) |

## Dateien
- `motifs.mjs`: die Zeichnung (Formen und Farben im 72er-Raster).
- `erzeuge-symbole.mjs`: schreibt `desk.svg`, `tasks.svg`, `board.svg` (Hauptquelle der Familie liegt im Desk-Repository unter `docs/brand`, hier eine Kopie) (Quellvektoren).
- Aufruf: `node docs/brand/erzeuge-symbole.mjs`

## PNG, ICO und ICNS erzeugen
Mit dem vorhandenen Tauri-Werkzeug, in `app/`:

```
npx tauri icon ../docs/brand/desk.svg -o src-tauri/icons
```

Danach die zusätzlich angelegten Ordner `android`, `ios` und die Datei `64x64.png` löschen (Desk braucht sie nicht). Für Tasks entsprechend `tasks.svg`; für Board gibt es noch keine App, nur die Quelle `board.svg`.
