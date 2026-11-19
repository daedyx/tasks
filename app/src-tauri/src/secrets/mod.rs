/// Dienstname im Windows-Tresor. Entwicklungs-Builds (`npm run tauri:dev`, Merkmal `dev-bridge`) haben einen
/// eigenen Dienst und Datenordner (`tauri.dev.conf.json`), damit Schema-Änderungen die installierte App nie berühren.
pub const SERVICE: &str = if cfg!(feature = "dev-bridge") { "daedyx-tasks-dev" } else { "daedyx-tasks" };

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
    pub fn new() -> Self {
        Self { service: SERVICE }
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

/// Speicher im Arbeitsspeicher, nur für Tests.
#[cfg(test)]
#[derive(Default)]
pub struct MemoryStore {
    map: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[cfg(test)]
impl SecretStore for MemoryStore {
    fn get(&self, key: &str) -> Result<Option<String>, SecretError> {
        Ok(self.map.lock().unwrap().get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), SecretError> {
        self.map.lock().unwrap().insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), SecretError> {
        self.map.lock().unwrap().remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_roundtrip() {
        let s = MemoryStore::default();
        assert_eq!(s.get("a").unwrap(), None);
        s.set("a", "1").unwrap();
        assert_eq!(s.get("a").unwrap(), Some("1".to_string()));
        s.delete("a").unwrap();
        assert_eq!(s.get("a").unwrap(), None);
        s.delete("a").unwrap(); // Löschen ohne Eintrag ist kein Fehler
    }

    // Echter Tresor: nur von Hand mit `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn keyring_store_roundtrip() {
        let s = KeyringStore::new();
        let k = "test-eintrag-daedyx-tasks";
        s.set(k, "wert").unwrap();
        assert_eq!(s.get(k).unwrap(), Some("wert".to_string()));
        s.delete(k).unwrap();
        assert_eq!(s.get(k).unwrap(), None);
    }
}
