//! Plan 4.2: Im Vorschau-Programm (ohne Merkmal `produkt`) stehen die echte Kennung `de.daedyx.tasks` und der echte
//! Tresor-Dienst `daedyx-tasks` nirgends, nur die der Vorschau. Geprüft an den Bytes des gebauten Programms.
//! Mit `produkt` (erst T7, auch bei `--all-features`) gilt das Gegenstück: echte Namen, keine der Vorschau.
const EXE: &str = env!("CARGO_BIN_EXE_daedyx-tasks-vorschau");

/// Stellen, an denen `needle` im Programm steht, je mit dem Rest dahinter.
fn hits<'a>(bytes: &'a [u8], needle: &[u8]) -> Vec<&'a [u8]> {
    bytes
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .filter_map(|(at, _)| bytes.get(at.saturating_add(needle.len())..))
        .collect()
}

/// Ein Test für beide Fassungen; `cfg!` statt `#[cfg(not(...))]`, damit Clippy beide Zweige sieht (keine
/// Ausnahme-Stelle) und `--all-features` nicht unerklärt rot ist.
#[test]
fn programm_traegt_nur_die_namen_seiner_fassung() {
    let bytes = std::fs::read(EXE).unwrap();
    if cfg!(feature = "produkt") {
        product(&bytes);
    } else {
        preview_counts(&bytes);
        preview_id(&bytes);
        preview_vault(&bytes);
    }
}

/// Zählprüfung (Review T2, N1-1): Jede Kennung ist die der Vorschau, und es gibt keinen losen Endungstext, der
/// hinter einer echten Kennung liegen und den Vergleich unten täuschen könnte. Deshalb endet auch kein Textschlüssel
/// auf `.vorschau` (früher `status.vorschau`, jetzt `status.hinweis`).
fn preview_counts(bytes: &[u8]) {
    let n = |t: &[u8]| hits(bytes, t).len();
    assert_eq!(
        n(b"de.daedyx.tasks"),
        n(b"de.daedyx.tasks.vorschau"),
        "echte Kennung"
    );
    assert_eq!(
        n(b".vorschau"),
        n(b"de.daedyx.tasks.vorschau"),
        "loser Text .vorschau"
    );
    assert_eq!(
        n(b"-vorschau"),
        n(b"daedyx-tasks-vorschau"),
        "loser Text -vorschau"
    );
}

fn preview_id(bytes: &[u8]) {
    let found = hits(bytes, b"de.daedyx.tasks");
    assert!(!found.is_empty(), "Vorschau-Kennung fehlt im Programm");
    for (n, rest) in found.iter().enumerate() {
        assert!(
            rest.starts_with(b".vorschau"),
            "echte Kennung, Fundstelle {n}"
        );
    }
}

/// `daedyx-tasks` kommt sonst nur als Teil von Crate-Namen vor (Pfade in Meldungen).
fn preview_vault(bytes: &[u8]) {
    assert!(
        !hits(bytes, b"daedyx-tasks-vorschau").is_empty(),
        "Vorschau-Tresor fehlt im Programm"
    );
    let allowed: [&[u8]; 5] = [b"-vorschau", b"-core", b"-ui", b"-store", b"-app"];
    for (n, rest) in hits(bytes, b"daedyx-tasks").iter().enumerate() {
        assert!(
            allowed.iter().any(|a| rest.starts_with(a)),
            "fremder Tresor-Name, Fundstelle {n}"
        );
    }
}

fn product(bytes: &[u8]) {
    assert!(
        hits(bytes, b"de.daedyx.tasks")
            .iter()
            .any(|r| !r.starts_with(b".vorschau"))
    );
    assert!(hits(bytes, b"de.daedyx.tasks.vorschau").is_empty());
    assert!(hits(bytes, b"daedyx-tasks-vorschau").is_empty());
}
