//! Kennungen der App (Plan 4.2), nur hier. Ohne das Merkmal `produkt` (Standard) gelten die der **Vorschau**; die
//! echten Kennungen sind dann nicht im Programm (Test `tests/kennung.rs` prüft das gebaute Programm). Nur technische
//! Kennungen, keine Oberflächentexte.
use daedyx_tasks_store::secrets::KeyringStore;

#[cfg(not(feature = "produkt"))]
mod names {
    pub const APP_ID: &str = "de.daedyx.tasks.vorschau";
    pub const VAULT: &str = "daedyx-tasks-vorschau";
}
#[cfg(feature = "produkt")]
mod names {
    pub const APP_ID: &str = "de.daedyx.tasks";
    pub const VAULT: &str = "daedyx-tasks";
}

/// Kennung der App (Datenordner, Fensterkennung, später App-Kennung für Benachrichtigungen).
pub const APP_ID: &str = names::APP_ID;
/// Dienst im Windows-Tresor (macOS: Schlüsselbund), Eintrag `db-key` kommt mit T5.
pub const VAULT: &str = names::VAULT;

/// Der Tresor dieser Fassung. Der Store bekommt den Dienst von hier, nie seinen festen `SERVICE`.
pub fn vault() -> KeyringStore {
    KeyringStore::for_service(VAULT)
}

/// Kennung und Tresor gehören zur selben Fassung (Vorschau oder Produkt). Die Startprobe prüft das.
/// `black_box` hält beide Namen im gebauten Programm, damit `tests/kennung.rs` sie dort sieht.
pub fn consistent() -> bool {
    let (id, vault) = std::hint::black_box((APP_ID, VAULT));
    id.ends_with(".vorschau") == vault.ends_with("-vorschau")
}
