/** Läuft die Oberfläche im Tauri-Fenster (und nicht im Browser-Vorschaufenster oder im Test)? */
export const isTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
