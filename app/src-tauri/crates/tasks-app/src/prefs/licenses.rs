//! Lizenzliste der Vorschau aus `lizenzen.tsv` (erzeugt von `scripts/vorschau-lizenzen.mjs` aus `cargo metadata`;
//! bei geänderten Abhängigkeiten neu erzeugen, ein Test vergleicht mit `Cargo.lock`).
const TABLE: &str = include_str!("../../lizenzen.tsv");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct License {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
}

pub fn licenses() -> Vec<License> {
    TABLE
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some(License {
                name: parts.next()?,
                version: parts.next()?,
                license: parts.next()?,
            })
        })
        .collect()
}

/// Treffer für den Filter (Name oder Lizenz, ohne Groß/klein).
pub fn matches(l: &License, filter: &str) -> bool {
    let f = filter.trim().to_lowercase();
    f.is_empty() || l.name.to_lowercase().contains(&f) || l.license.to_lowercase().contains(&f)
}
