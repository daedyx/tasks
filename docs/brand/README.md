# Symbole der Daedyx-Apps

Eigene Zeichnung, entstanden aus Code (keine Bildgeneratoren, keine fremden Vorlagen, keine Piktogramm-Sammlung). Richtung „Farbe und Motiv“ (Björn, 2026-11-21): gleiche Kachelform wie der Launcher, jede App mit eigener Farbe und eigenem Motiv, gemeinsam Indigo, Bernstein und das Raster („Labyrinth-D“, `daedyx-launcher`). Urheberschaft: Daedyx contributors, Lizenz wie das Projekt (MIT).

| App | Motiv |
|---|---|
| Desk | Indigo, Postfach mit Kalenderringen |
| Tasks | Bernstein, ein großer Haken |
| Board | Petrol, zwei schräge Karten mit Markerstrich (Name „Board“ ist ein Arbeitsname; das Repository fehlt noch) |

## Dateien
- `motifs.mjs`: die Zeichnung (Formen und Farben im 72er-Raster).
- `erzeuge-symbole.mjs`: schreibt `desk.svg`, `tasks.svg`, `board.svg` (Quellvektoren).
- Aufruf: `node docs/brand/erzeuge-symbole.mjs`

## PNG, ICO und ICNS erzeugen
Mit dem vorhandenen Tauri-Werkzeug, in `app/`:

```
npx tauri icon ../docs/brand/desk.svg -o src-tauri/icons
```

Danach die zusätzlich angelegten Ordner `android`, `ios` und die Datei `64x64.png` löschen (Desk braucht sie nicht). Für Tasks entsprechend `tasks.svg`; für Board gibt es noch keine App, nur die Quelle `board.svg`.
