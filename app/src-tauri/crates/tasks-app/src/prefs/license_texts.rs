//! Lizenztexte der Bibliotheken aus `lizenztexte.txt` (erzeugt von `scripts/vorschau-lizenzen.mjs`): je Eintrag
//! ein Abschnitt `== Name Version ==` mit Lizenz, Urheber und Verweisen `[Tn]` auf die Texte im Anhang
//! (`--- [Tn] ---`). Die Seite „Über“ zeigt daraus den vollständigen Hinweis; ein Test erkennt Lücken.
#[cfg(test)]
use super::licenses::License;

pub const TEXTS: &str = include_str!("../../lizenztexte.txt");

const APPENDIX: &str = "=== Lizenztexte ===";

/// Abschnitt eines Eintrags (ohne Kopfzeile), bis zur nächsten Leerzeile.
fn section<'a>(name: &str, version: &str, all: &'a str) -> Option<&'a str> {
    let head = format!("== {name} {version} ==\n");
    let start = all.find(&head)?.checked_add(head.len())?;
    let rest = all.get(start..)?;
    let end = rest.find("\n\n").unwrap_or(rest.len());
    rest.get(..end)
}

/// Text zu `[Tn]` aus dem Anhang, ohne die Kopfzeile.
fn text<'a>(id: &str, all: &'a str) -> Option<&'a str> {
    let appendix = all.get(all.find(APPENDIX)?..)?;
    let head = format!("--- {id} ---\n");
    let start = appendix.find(&head)?.checked_add(head.len())?;
    let rest = appendix.get(start..)?;
    let end = rest.find("\n--- [").unwrap_or(rest.len());
    let body = rest.get(..end)?.trim();
    (!body.is_empty()).then_some(body)
}

fn refs(section: &str) -> Vec<&str> {
    section
        .lines()
        .find_map(|l| l.strip_prefix("texte: "))
        .map(|r| r.split_whitespace().collect())
        .unwrap_or_default()
}

/// Vollständiger Hinweis eines Eintrags: Lizenz, Urheber und alle Lizenztexte.
pub fn full_text(name: &str, version: &str, all: &str) -> Option<String> {
    let s = section(name, version, all)?;
    let mut out = s
        .lines()
        .filter(|l| !l.starts_with("texte: "))
        .collect::<Vec<_>>()
        .join("\n");
    for id in refs(s) {
        out.push_str("\n\n");
        out.push_str(text(id, all)?);
    }
    Some(out)
}

/// Was einem Eintrag fehlt (nur für den Test der Vollständigkeit).
#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gap {
    Section,
    Text,
    Reference(String),
    Author,
}

#[cfg(test)]
/// Einträge ohne Abschnitt, ohne Verweis, mit Verweis ins Leere oder ohne Urheber: (Name, Version, Lücke).
pub fn missing(list: &[License], all: &str) -> Vec<(String, String, Gap)> {
    let mut gaps = Vec::new();
    for l in list {
        let gap = match section(l.name, l.version, all) {
            None => Some(Gap::Section),
            Some(s) => {
                let ids = refs(s);
                if ids.is_empty() {
                    Some(Gap::Text)
                } else if let Some(id) = ids.iter().find(|id| text(id, all).is_none()) {
                    Some(Gap::Reference((*id).to_owned()))
                } else if !s.lines().any(|line| line.starts_with("urheber: ")) {
                    Some(Gap::Author)
                } else {
                    None
                }
            }
        };
        if let Some(g) = gap {
            gaps.push((l.name.to_owned(), l.version.to_owned(), g));
        }
    }
    gaps
}
