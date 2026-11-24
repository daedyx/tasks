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

// Echter Tresor: nur von Hand mit `cargo test -- --ignored`. Benutzt einen eigenen Test-Eintrag, nie `db-key`.
#[test]
#[ignore = "echter Windows-Tresor, nur von Hand (SICHERHEIT-AUSNAHMEN)"]
fn keyring_store_roundtrip() {
    let s = KeyringStore::new();
    let k = "test-eintrag-daedyx-tasks";
    s.set(k, "wert").unwrap();
    assert_eq!(s.get(k).unwrap(), Some("wert".to_string()));
    s.delete(k).unwrap();
    assert_eq!(s.get(k).unwrap(), None);
}
