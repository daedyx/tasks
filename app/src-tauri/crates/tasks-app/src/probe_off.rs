//! Produkt-Fassung: keine Installationsprobe (Gegenstück zu `probe.rs`, das nur die Vorschau übersetzt, A-034). Ein
//! leerer Platzhalter, damit `TasksApp` und `logic` in beiden Fassungen gleich aussehen; er hält kein Setup und ruft
//! nichts auf (Review U2-R N-U2-1, Wächter `update_net_tests`).

/// Immer leer.
#[derive(Debug, Default)]
pub struct Probe {}

impl Probe {
    /// Nie die Umgebung einer Probe: Die Update-Suche läuft wie eingestellt.
    pub fn quiet(&self) -> bool {
        false
    }
}

impl crate::app::TasksApp {
    /// Tut nichts.
    pub(crate) fn run_update_probe(&mut self) {}
}
