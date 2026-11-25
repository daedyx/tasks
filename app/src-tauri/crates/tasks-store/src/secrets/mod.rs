//! Tresor für den Datenbank-Schlüssel. Produkt: Windows-Tresor bzw. macOS-Schlüsselbund (`KeyringStore`);
//! Tests: `MemoryStore` (nur mit `cfg(test)` oder Merkmal `test-support`, nie im Produkt-Bau).

/// Dienstname im Windows-Tresor. Entwicklungs-Builds (`npm run tauri:dev`, Merkmal `dev-vault` über `dev-bridge`) haben
/// einen eigenen Dienst und Datenordner (`tauri.dev.conf.json`), damit Schema-Änderungen die installierte App nie berühren.
pub const SERVICE: &str = if cfg!(feature = "dev-vault") { "daedyx-tasks-dev" } else { "daedyx-tasks" };

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("Tresor-Zugriff fehlgeschlagen: {0}")]
    Store(String),
}

pub trait SecretStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, SecretError>;
    fn set(&self, key: &str, value: &str) -> Result<(), SecretError>;
    fn delete(&self, key: &str) -> Result<(), SecretError>;
}

/// Windows-Tresor (macOS: Schlüsselbund) über die Bibliothek `keyring`.
pub struct KeyringStore {
    service: &'static str,
}

impl KeyringStore {
    /// Dienst `SERVICE` (Tauri-Hülle).
    pub fn new() -> Self {
        Self::for_service(SERVICE)
    }

    /// Dienst, den die App vorgibt. Die Rust-Vorschau übergibt ihren eigenen (`daedyx-tasks-vorschau`, Plan 4.2), damit
    /// sie den echten Tresor-Eintrag nie liest.
    pub fn for_service(service: &'static str) -> Self {
        Self { service }
    }

    pub fn service(&self) -> &'static str {
        self.service
    }
}

impl Default for KeyringStore {
    fn default() -> Self {
        Self::new()
    }
}

fn entry(service: &str, key: &str) -> Result<keyring::Entry, SecretError> {
    keyring::Entry::new(service, key).map_err(|e| SecretError::Store(e.to_string()))
}

impl SecretStore for KeyringStore {
    fn get(&self, key: &str) -> Result<Option<String>, SecretError> {
        match entry(self.service, key)?.get_password() {
            Ok(v) => Ok(Some(v)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(SecretError::Store(e.to_string())),
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), SecretError> {
        entry(self.service, key)?.set_password(value).map_err(|e| SecretError::Store(e.to_string()))
    }

    fn delete(&self, key: &str) -> Result<(), SecretError> {
        match entry(self.service, key)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(SecretError::Store(e.to_string())),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
mod memory;
#[cfg(any(test, feature = "test-support"))]
pub use memory::MemoryStore;

#[cfg(test)]
mod tests;
