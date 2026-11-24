use super::DbError;
use crate::secrets::SecretStore;

pub const KEY_NAME: &str = "db-key";

fn is_valid(key: &str) -> bool {
    key.len() == 64 && key.chars().all(|c| c.is_ascii_hexdigit())
}

/// Holt den Datenbankschlüssel (64 Hex-Zeichen) aus dem Tresor oder erzeugt und speichert einen neuen.
/// Ein vorhandener, aber ungültiger Wert wird nie überschrieben (`BadKey`).
pub fn get_or_create(secrets: &dyn SecretStore) -> Result<String, DbError> {
    if let Some(existing) = secrets.get(KEY_NAME)? {
        return if is_valid(&existing) { Ok(existing) } else { Err(DbError::BadKey) };
    }
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| DbError::Random(e.to_string()))?;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    secrets.set(KEY_NAME, &hex)?;
    // Zurücklesen: nur was wirklich im Tresor steht, darf die Datenbank verschlüsseln.
    match secrets.get(KEY_NAME)? {
        Some(stored) if is_valid(&stored) => Ok(stored),
        _ => Err(DbError::BadKey),
    }
}
