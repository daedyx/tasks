//! Kennungen der App (Plan 4.2), nur hier. Ohne das Merkmal `produkt` (Standard) gelten die der **Vorschau**; die
//! echte Kennung ist dann nicht im Programm (Test `tests/kennung.rs` prüft das gebaute Programm). Nur technische
//! Kennungen, keine Oberflächentexte.

/// Kennung der App (Datenordner, Fensterkennung, später App-Kennung für Benachrichtigungen).
#[cfg(not(feature = "produkt"))]
pub const APP_ID: &str = "de.daedyx.tasks.vorschau";
#[cfg(feature = "produkt")]
pub const APP_ID: &str = "de.daedyx.tasks";
