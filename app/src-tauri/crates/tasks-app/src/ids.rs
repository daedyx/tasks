//! Kennungen der App (Plan 4.2), nur hier. Ohne das Merkmal `produkt` (Standard) gelten die der **Vorschau**; die
//! echten Kennungen sind dann nicht im Programm (Test `tests/kennung.rs` prüft das gebaute Programm). Nur technische
//! Kennungen, keine Oberflächentexte.
use daedyx_background::ids::AppIds;
use daedyx_tasks_store::secrets::KeyringStore;

#[cfg(not(feature = "produkt"))]
mod names {
    pub const APP_ID: &str = "de.daedyx.tasks.vorschau";
    pub const VAULT: &str = "daedyx-tasks-vorschau";
    pub const RUN_VALUE: &str = "DaedyxTasksVorschau";
    pub const PIPE_PREFIX: &str = "daedyx-tasks-vorschau-instanz";
    pub const PREVIEW: bool = true;
}
#[cfg(feature = "produkt")]
mod names {
    pub const APP_ID: &str = "de.daedyx.tasks";
    pub const VAULT: &str = "daedyx-tasks";
    pub const RUN_VALUE: &str = "DaedyxTasks";
    pub const PIPE_PREFIX: &str = "daedyx-tasks-instanz";
    pub const PREVIEW: bool = false;
}

/// Kennung der App (Datenordner, Fensterkennung, später App-Kennung für Benachrichtigungen).
pub const APP_ID: &str = names::APP_ID;
/// Dienst im Windows-Tresor (macOS: Schlüsselbund), Eintrag `db-key` kommt mit T5.
pub const VAULT: &str = names::VAULT;

/// Name des Autostart-Werts unter `HKCU\...\Run` (T4).
pub const RUN_VALUE: &str = names::RUN_VALUE;
/// Anfang des Pipe-Namens der Einzelinstanz (dahinter ein Hash des Benutzers, T4).
pub const PIPE_PREFIX: &str = names::PIPE_PREFIX;

/// Kennungen für den Hintergrund: App-Kennung der Benachrichtigungen ist die Kennung der App.
pub const BACKGROUND: AppIds = AppIds {
    aumid: APP_ID,
    run_value: RUN_VALUE,
    pipe_prefix: PIPE_PREFIX,
};

/// Der Tresor dieser Fassung. Der Store bekommt den Dienst von hier, nie seinen festen `SERVICE`.
pub fn vault() -> KeyringStore {
    KeyringStore::for_service(VAULT)
}

/// Kennung und Tresor gehören zur selben Fassung (Vorschau oder Produkt). Die Startprobe prüft das.
/// `black_box` hält beide Namen im gebauten Programm, damit `tests/kennung.rs` sie dort sieht. Bewusst ohne
/// einzelne Endungstexte wie „.vorschau“: Rust legt Texte lückenlos hintereinander, ein loses „.vorschau“ hinter
/// einer echten Kennung würde den Programmtest täuschen (Review T2, N1-1). Verglichen wird das letzte Glied beider
/// Namen miteinander („vorschau“/„vorschau“ oder „tasks“/„tasks“) und mit `PREVIEW`.
/// Autostart-Wert und Pipe gehören ebenfalls dazu: Der Autostart-Wert ist der Tresor-Name ohne Bindestriche (Groß-
/// und Kleinschreibung egal), die Pipe beginnt mit dem Tresor-Namen und einem Bindestrich. Auch hier ohne
/// Endungstexte; „Vorschau“ steht nur in den vollständigen Namen.
pub fn consistent() -> bool {
    let (id, vault, preview) = std::hint::black_box((APP_ID, VAULT, names::PREVIEW));
    let (run, pipe) = std::hint::black_box((RUN_VALUE, PIPE_PREFIX));
    same_version(id, vault, preview) && same_background(vault, run, pipe)
}

/// Der Vergleich der Hintergrund-Namen mit dem Tresor-Namen (testbar für gemischte Fassungen).
pub fn same_background(vault: &str, run: &str, pipe: &str) -> bool {
    let joined: String = vault.chars().filter(|c| *c != '-').collect();
    let pipe_rest = pipe.strip_prefix(vault).and_then(|r| r.strip_prefix('-'));
    run.eq_ignore_ascii_case(&joined)
        && pipe_rest.is_some_and(|r| !r.is_empty() && !r.contains('-'))
}

/// Der Vergleich von `consistent` für beliebige Namen (testbar auch für gemischte Fassungen).
pub fn same_version(id: &str, vault: &str, preview: bool) -> bool {
    let (id_end, vault_end) = (id.rsplit('.').next(), vault.rsplit('-').next());
    id_end == vault_end && (id_end == Some("tasks")) != preview
}
