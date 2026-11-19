//! Hintergrundlauf: Start im Infobereich (Autostart) und Einzelinstanz.
//! Symbol, Menü und „Fenster schließen versteckt nur“ liegen in der Oberfläche (`src/background`), damit die Texte in `de.json` bleiben.

/// Argument, mit dem der Autostart die App ohne sichtbares Fenster startet.
pub const HIDDEN_ARG: &str = "--hidden";

pub fn launched_hidden(args: &[String]) -> bool {
    args.iter().any(|a| a == HIDDEN_ARG)
}
