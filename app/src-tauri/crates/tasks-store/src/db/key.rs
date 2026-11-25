use super::DbError;
use crate::secrets::SecretStore;
use zeroize::Zeroizing;

pub const KEY_NAME: &str = "db-key";

fn is_valid(key: &str) -> bool {
    key.len() == 64 && key.chars().all(|c| c.is_ascii_hexdigit())
}

/// Holt den Datenbankschlüssel (64 Hex-Zeichen) aus dem Tresor oder erzeugt und speichert einen neuen.
/// Ein vorhandener, aber ungültiger Wert wird nie überschrieben (`BadKey`). Alle Kopien liegen in `Zeroizing`
/// (Review T5, N5-2); was `keyring` und SQLite intern kopieren, bleibt außerhalb unserer Hand.
pub fn get_or_create(secrets: &dyn SecretStore) -> Result<Zeroizing<String>, DbError> {
    if let Some(existing) = secrets.get(KEY_NAME)?.map(Zeroizing::new) {
        return if is_valid(&existing) { Ok(existing) } else { Err(DbError::BadKey) };
    }
    let mut bytes = Zeroizing::new([0u8; 32]);
    getrandom::fill(&mut bytes[..]).map_err(|e| DbError::Random(e.to_string()))?;
    // Feste Größe vorab: kein Umkopieren beim Wachsen, das Reste im freigegebenen Speicher ließe.
    let mut hex = Zeroizing::new(String::with_capacity(64));
    for b in bytes.iter() {
        for nibble in [b >> 4, b & 0x0f] {
            hex.push(char::from_digit(u32::from(nibble), 16).unwrap_or('0'));
        }
    }
    secrets.set(KEY_NAME, &hex)?;
    // Zurücklesen: nur was wirklich im Tresor steht, darf die Datenbank verschlüsseln.
    match secrets.get(KEY_NAME)?.map(Zeroizing::new) {
        Some(stored) if is_valid(&stored) => Ok(stored),
        _ => Err(DbError::BadKey),
    }
}

/// Wert für `PRAGMA key` (`x'<hex>'`), ebenfalls in `Zeroizing` und ohne Umkopieren gebaut.
pub fn pragma(hex: &str) -> Zeroizing<String> {
    let mut text = Zeroizing::new(String::with_capacity(hex.len().saturating_add(3)));
    text.push_str("x'");
    text.push_str(hex);
    text.push('\'');
    text
}
