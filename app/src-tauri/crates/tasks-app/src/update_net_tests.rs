//! Netz nur für die Update-Suche (Ausnahme U-A4, plan-update-bibliothek 9, Review U2-R N-U2-5) und die Weiche der
//! Installationsprobe (N-U2-1). Ohne Netz: eine Attrappe zählt jede Anfrage; die Abhängigkeiten liest der Test aus
//! `Cargo.lock`, die Weiche aus dem Quelltext.
use crate::update::config;
use daedyx_updater::daedyx_update::attest::{AttestError, Provenance};
use daedyx_updater::daedyx_update::fetch::{HttpsUrl, Reply, Transport, TransportError};
use daedyx_updater::{Backend, Status, Updater};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;

/// Zählt Anfragen und antwortet immer mit 404.
struct Counting(Sender<String>);

impl Transport for Counting {
    fn get(&mut self, url: &HttpsUrl) -> Result<Reply<'_>, TransportError> {
        let _ = self.0.send(url.as_str().to_owned());
        Ok(Reply::Status(404))
    }
}

struct Never;

impl Provenance for Never {
    fn verify(&self, _: &[u8; 32], _: &[u8]) -> Result<(), AttestError> {
        Err(AttestError {
            kind: daedyx_updater::daedyx_update::attest::AttestKind::Invalid,
            detail: "attrappe".into(),
        })
    }
}

/// Entscheid F-U2-1 und Netzgate (U-A4): Die Vorschau sucht nicht von selbst, also null Verbindungen, solange
/// niemand sucht. Die Produkt-Fassung sucht beim Start (höchstens einmal am Tag).
#[test]
fn ohne_suche_keine_verbindung_und_die_vorschau_sucht_nicht_von_selbst() {
    let base = crate::test_support::tmp("u2-netzgate");
    let dir = crate::paths::data_dir_in(&base).unwrap();
    let mut c = config(&dir, &base).unwrap();
    assert_eq!(c.auto_check_default, !crate::ids::PREVIEW);
    if !crate::ids::PREVIEW {
        return;
    }
    c.auto_delay = Duration::ZERO;
    let (tx, rx) = channel();
    let backend = Backend {
        transport: Box::new(Counting(tx)),
        provenance: Box::new(|_| Ok(Box::new(Never) as Box<dyn Provenance>)),
        free_space: Box::new(|_: &Path| None),
        now: Box::new(|| 1_800_000_000),
    };
    let mut u = Updater::start(c, backend, None);
    std::thread::sleep(Duration::from_millis(400));
    u.poll();
    assert!(!u.auto_check(), "Schalter „Beim Start suchen“ steht aus");
    assert_eq!(u.status(), &Status::Idle);
    assert_eq!(rx.try_iter().count(), 0, "keine Verbindung ohne Suche");
    u.check_now();
    assert!(
        rx.recv_timeout(Duration::from_secs(10)).is_ok(),
        "erst der Klick sucht"
    );
}

/// Abhängigkeiten aus `Cargo.lock`: Paketname → Namen seiner Abhängigkeiten.
fn lock_graph() -> BTreeMap<String, BTreeSet<String>> {
    let lock =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.lock"))
            .unwrap();
    let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for block in lock.split("[[package]]").skip(1) {
        let name = block
            .lines()
            .find_map(|l| l.strip_prefix("name = \""))
            .map(|n| n.trim_end_matches('"').to_owned())
            .unwrap();
        let deps = block
            .lines()
            .filter_map(|l| l.strip_prefix(" \""))
            .map(|d| {
                d.trim_end_matches("\",")
                    .split(' ')
                    .next()
                    .unwrap()
                    .to_owned()
            });
        graph.entry(name).or_default().extend(deps);
    }
    graph
}

fn reachable(graph: &BTreeMap<String, BTreeSet<String>>, skip: &str) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut todo = vec!["daedyx-tasks-app".to_owned()];
    while let Some(p) = todo.pop() {
        if p != skip && seen.insert(p.clone()) {
            todo.extend(graph.get(&p).into_iter().flatten().cloned());
        }
    }
    seen
}

/// Netz-Wächter (U-A4): Netz-Crates erreicht das Programm nur über `daedyx-updater`. Die Tauri-Hülle im selben
/// Workspace hat ihr eigenes Netz und zählt hier nicht.
#[test]
fn netz_nur_ueber_den_updater() {
    let net = [
        "ureq",
        "rustls",
        "reqwest",
        "hyper",
        "h2",
        "tokio-rustls",
        "webpki-roots",
        "rustls-platform-verifier",
        "native-tls",
        "tungstenite",
        "curl",
    ];
    let graph = lock_graph();
    let all = reachable(&graph, "");
    let without = reachable(&graph, "daedyx-updater");
    assert!(all.contains("ureq"), "Probe: der Updater bringt ureq mit");
    let leaked: Vec<&str> = net.into_iter().filter(|c| without.contains(*c)).collect();
    assert!(
        leaked.is_empty(),
        "Netz an daedyx-updater vorbei: {leaked:?}"
    );
}

/// Weiche der Probe (N-U2-1): Der Weg ohne Nachweis steht nur in `probe.rs` (nicht in der Produkt-Fassung); das
/// Gegenstück `probe_off.rs` kann nichts starten und hält kein Setup.
#[test]
fn pruefweg_ohne_nachweis_nur_in_der_vorschau() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for entry in std::fs::read_dir(&src).unwrap().filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "probe.rs" || name.ends_with("_tests.rs") || name == "test_support.rs" {
            continue;
        }
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        for word in [
            "file_sha256",
            "run_update_probe_with",
            "DAEDYX_UPDATE_PROBE",
            "setup_args",
        ] {
            assert!(!text.contains(word), "{word} in {name}");
        }
    }
    let off = std::fs::read_to_string(src.join("probe_off.rs")).unwrap();
    for word in ["daedyx_sys", "start", "PathBuf", "std::env"] {
        assert!(!off.contains(word), "{word} in probe_off.rs");
    }
    let lib = std::fs::read_to_string(src.join("lib.rs"))
        .unwrap()
        .replace("\r\n", "\n");
    assert!(lib.contains("#[cfg(not(feature = \"produkt\"))]\npub mod probe;"));
}
