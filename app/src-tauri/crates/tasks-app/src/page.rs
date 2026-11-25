//! Seiten der Navigationsleiste (Entscheid T-F5: Liste, Board, Einstellungen; keine Seite „Erinnerungen“ in v1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    List,
    Board,
    Settings,
}

impl Page {
    /// Reihenfolge in der Leiste; Einstellungen stehen gesondert ganz unten.
    pub const MAIN: [Page; 2] = [Page::List, Page::Board];

    pub fn key(self) -> &'static str {
        match self {
            Page::List => "nav.liste",
            Page::Board => "nav.board",
            Page::Settings => "nav.einstellungen",
        }
    }
}
