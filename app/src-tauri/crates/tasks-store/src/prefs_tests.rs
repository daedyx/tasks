//! Einstellungen in `task_meta` (Plan 4.4): Positivliste, Werte kurz und ohne Steuerzeichen, die Schlüssel des Kerns
//! (Schema-Stand, Gerätekennung, Uhrstand) bleiben unerreichbar. Missbrauch nach Plan 5.3 (`prefs_value`) mit einem
//! festen Zufallsgenerator ohne neue Abhängigkeit. Nur Speicher-Datenbanken.
use super::*;

fn conn() -> Connection {
    let c = Connection::open_in_memory().unwrap();
    daedyx_tasks_core::migrate(&c).unwrap();
    c
}

#[test]
fn erlaubte_schluessel_lassen_sich_schreiben_lesen_und_entfernen() {
    let c = conn();
    for key in KEYS {
        set(&c, key, "wert").unwrap();
        assert_eq!(get(&c, key).as_deref(), Some("wert"), "{key}");
        set(&c, key, "neu").unwrap();
        assert_eq!(get(&c, key).as_deref(), Some("neu"), "{key}");
        remove(&c, key).unwrap();
        assert_eq!(get(&c, key), None, "{key}");
    }
    set(&c, "app.board_mode.0123abcd", "board").unwrap();
    assert_eq!(board_modes(&c), [("0123abcd".to_string(), "board".to_string())]);
}

#[test]
fn schluessel_des_kerns_und_fremde_bleiben_unerreichbar() {
    let c = conn();
    let before: String =
        c.query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).unwrap();
    for key in [
        "schema_version",
        "device_id",
        "hlc",
        "app.",
        "app.theme.x",
        "app.board_mode.",
        "app.board_mode.a b",
        "app.board_mode.%",
        "app.board_mode.../x",
        "APP.THEME",
        " app.theme",
    ] {
        assert_eq!(set(&c, key, "1"), Err(PrefsError::Key), "{key:?}");
        assert_eq!(remove(&c, key), Err(PrefsError::Key), "{key:?}");
        assert_eq!(get(&c, key), None, "{key:?}");
    }
    let after: String =
        c.query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).unwrap();
    assert_eq!(before, after);
    let long_id = format!("app.board_mode.{}", "a".repeat(65));
    assert_eq!(set(&c, &long_id, "board"), Err(PrefsError::Key));
}

#[test]
fn zu_lange_werte_und_steuerzeichen_werden_abgelehnt_und_beim_lesen_ignoriert() {
    let c = conn();
    assert_eq!(set(&c, "app.theme", &"x".repeat(MAX_VALUE + 1)), Err(PrefsError::Value));
    assert_eq!(set(&c, "app.theme", "dark\0"), Err(PrefsError::Value));
    assert_eq!(set(&c, "app.theme", "a\nb"), Err(PrefsError::Value));
    set(&c, "app.theme", &"ä".repeat(MAX_VALUE)).unwrap();
    // Was eine andere Fassung trotzdem hineingeschrieben hat, liest sich als „nicht gesetzt“.
    c.execute("UPDATE task_meta SET value = 'a' || char(7) WHERE key = 'app.theme'", []).unwrap();
    assert_eq!(get(&c, "app.theme"), None);
    c.execute("INSERT INTO task_meta (key, value) VALUES ('app.board_mode.zz', char(0))", []).unwrap();
    assert!(board_modes(&c).is_empty());
}

/// Fester Zufallsgenerator (xorshift64*), damit ein Fund wiederholbar ist.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn text(&mut self, max: usize) -> String {
        let len = usize::try_from(self.next().checked_rem((max as u64).wrapping_add(1)).unwrap_or(0)).unwrap();
        (0..len)
            .map(|_| {
                let pick = self.next() % 5;
                let code = match pick {
                    0 => self.next() % 0x20,
                    1 => (self.next() % 0x60).wrapping_add(0x20),
                    2 => (self.next() % 0x2000).wrapping_add(0xA0),
                    3 => (self.next() % 0x50).wrapping_add(0x1F600),
                    _ => [0x202E, 0xFEFF, 0x0, 0x7F, 0x2028][usize::try_from(self.next() % 5).unwrap()],
                };
                char::from_u32(u32::try_from(code).unwrap()).unwrap_or('?')
            })
            .collect()
    }
}

/// Plan 5.3 `prefs_value`: beliebige Schlüssel und Werte führen nie zu einer Panik; geschrieben wird nur, was die
/// Positivliste und die Wertregel erlauben, und genau das kommt wieder heraus.
#[test]
fn beliebige_schluessel_und_werte_bleiben_harmlos() {
    let c = conn();
    let mut rng = Rng(0x00D3_ADB3_3F00_0005);
    let prefixes = ["app.theme", "app.board_mode.", "app.", "schema_version", "", "app.language"];
    for _ in 0..3000 {
        let prefix = prefixes[usize::try_from(rng.next() % prefixes.len() as u64).unwrap()];
        let key = if rng.next().is_multiple_of(2) { prefix.to_string() } else { format!("{prefix}{}", rng.text(70)) };
        let value = rng.text(260);
        match set(&c, &key, &value) {
            Ok(()) => {
                assert!(allowed(&key) && value.chars().count() <= MAX_VALUE, "{key:?}");
                assert_eq!(get(&c, &key), Some(value));
            }
            Err(PrefsError::Key) => assert!(!allowed(&key), "{key:?}"),
            Err(PrefsError::Value) => assert!(allowed(&key), "{key:?}"),
            Err(PrefsError::Db(e)) => panic!("Datenbankfehler: {e}"),
        }
    }
    let version: String =
        c.query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).unwrap();
    assert_eq!(version, daedyx_tasks_core::SCHEMA_VERSION.to_string());
}
