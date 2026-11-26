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
        preview_background(&bytes);
        // Installationsprobe des Updates (U2): Gegenprobe zur Produkt-Fassung, die sie nicht enthält.
        assert!(!hits(&bytes, b"DAEDYX_UPDATE_PROBE").is_empty());
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

/// `daedyx-tasks` kommt sonst nur als Teil von Crate-Namen vor (Pfade in Meldungen). Seit T5 stehen auch die
/// Dateinamen `daedyx-tasks.db` und `daedyx-tasks-v…` (Sicherungen) im Programm. Am Byte lässt sich ein echter
/// Tresor-Name, hinter dem zufällig ein Text `.db…` oder `-v…` liegt, davon nicht unterscheiden. Deshalb ist die
/// Anzahl dieser Fundstellen festgeschrieben (Review T5, N5-3): Kommt eine dazu, wird der Test rot, und ein Mensch
/// sieht nach, ob es wirklich ein Dateiname ist.
const DB_NAMES: usize = 2;
const BACKUP_NAMES: usize = 3;

fn preview_vault(bytes: &[u8]) {
    let n = |t: &[u8]| hits(bytes, t).len();
    assert!(
        n(b"daedyx-tasks-vorschau") > 0,
        "Vorschau-Tresor fehlt im Programm"
    );
    assert_eq!(
        n(b"daedyx-tasks.db"),
        DB_NAMES,
        "Dateiname der Datenbank (neu gezählt?)"
    );
    assert_eq!(
        n(b"daedyx-tasks-v").saturating_sub(n(b"daedyx-tasks-vorschau")),
        BACKUP_NAMES,
        "Dateiname der Sicherungen (neu gezählt?)"
    );
    let allowed: [&[u8]; 7] = [
        b"-vorschau",
        b"-core",
        b"-ui",
        b"-store",
        b"-app",
        b".db",
        b"-v",
    ];
    for (n, rest) in hits(bytes, b"daedyx-tasks").iter().enumerate() {
        assert!(
            allowed.iter().any(|a| rest.starts_with(a)),
            "fremder Tresor-Name, Fundstelle {n}"
        );
    }
}

/// Autostart-Wert und Pipe der Vorschau (T4): `DaedyxTasks` nur als `DaedyxTasksVorschau`, gleich oft (kein loser
/// Text dahinter, der eine echte Kennung verdecken könnte), und die Pipe beginnt mit dem Tresor-Namen der Vorschau.
fn preview_background(bytes: &[u8]) {
    let n = |t: &[u8]| hits(bytes, t).len();
    assert!(
        n(b"DaedyxTasksVorschau") > 0,
        "Autostart-Wert fehlt im Programm"
    );
    assert_eq!(
        n(b"DaedyxTasks"),
        n(b"DaedyxTasksVorschau"),
        "echter Autostart-Wert"
    );
    assert!(
        n(b"daedyx-tasks-vorschau-instanz") > 0,
        "Pipe der Vorschau fehlt im Programm"
    );
    assert_eq!(n(b"tasks-instanz"), 0, "echte Pipe");
}

fn product(bytes: &[u8]) {
    assert!(
        hits(bytes, b"de.daedyx.tasks")
            .iter()
            .any(|r| !r.starts_with(b".vorschau"))
    );
    assert!(hits(bytes, b"de.daedyx.tasks.vorschau").is_empty());
    assert!(hits(bytes, b"daedyx-tasks-vorschau").is_empty());
    assert!(hits(bytes, b"DaedyxTasksVorschau").is_empty());
    assert!(!hits(bytes, b"DaedyxTasks").is_empty());
    // Pipe der Produkt-Fassung (Review T4, H4-3), keine der Vorschau.
    assert!(!hits(bytes, b"daedyx-tasks-instanz").is_empty());
    assert!(hits(bytes, b"daedyx-tasks-vorschau-instanz").is_empty());
    // Installationsprobe des Updates (U2, ohne Nachweis) gibt es nur in der Vorschau.
    assert!(hits(bytes, b"DAEDYX_UPDATE_PROBE").is_empty());
}
