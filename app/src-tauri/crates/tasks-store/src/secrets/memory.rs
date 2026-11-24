//! Tresor im Arbeitsspeicher, nur für Tests (eigene und die anderer Crates über das Merkmal `test-support`).
use super::{SecretError, SecretStore};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Default)]
pub struct MemoryStore {
    map: Mutex<HashMap<String, String>>,
}

impl MemoryStore {
    /// Eine Panik in einem anderen Test darf die Tabelle nicht unbrauchbar machen.
    fn map(&self) -> MutexGuard<'_, HashMap<String, String>> {
        self.map.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl SecretStore for MemoryStore {
    fn get(&self, key: &str) -> Result<Option<String>, SecretError> {
        Ok(self.map().get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), SecretError> {
        self.map().insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), SecretError> {
        self.map().remove(key);
        Ok(())
    }
}
