//! Wächter (plan-tasks-rust T0, 5.1): Das Store-Crate hängt weder an Tauri noch am Netz. Geprüft wird die volle
//! Abhängigkeits-Hülle laut `Cargo.lock` (alle Zielsysteme, auch Bau-Abhängigkeiten), ohne einen Prozess zu starten.
use std::collections::{BTreeMap, BTreeSet};

/// Paketname → Namen der direkten Abhängigkeiten (ohne Versionsangabe).
fn lock_graph() -> BTreeMap<String, Vec<String>> {
    let lock = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock")).unwrap();
    let mut graph = BTreeMap::new();
    for block in lock.split("[[package]]").skip(1) {
        let name = block.lines().find_map(|l| l.strip_prefix("name = \"")).unwrap().trim_end_matches('"').to_string();
        let deps: Vec<String> = match block.split_once("dependencies = [") {
            Some((_, rest)) => rest
                .split(']')
                .next()
                .unwrap()
                .lines()
                .filter_map(|l| l.trim().strip_prefix('"'))
                .map(|d| d.trim_end_matches(['"', ',']).split(' ').next().unwrap().to_string())
                .collect(),
            None => Vec::new(),
        };
        graph.entry(name).or_insert_with(Vec::new).extend(deps);
    }
    graph
}

fn closure(root: &str) -> BTreeSet<String> {
    let graph = lock_graph();
    assert!(graph.contains_key(root), "{root} fehlt in Cargo.lock");
    let mut seen = BTreeSet::new();
    let mut todo = vec![root.to_string()];
    while let Some(name) = todo.pop() {
        if seen.insert(name.clone()) {
            todo.extend(graph.get(&name).cloned().unwrap_or_default());
        }
    }
    seen
}

#[test]
fn the_store_has_no_tauri_and_no_network_in_its_tree() {
    let all = closure("daedyx-tasks-store");
    assert!(all.contains("rusqlite") && all.contains("daedyx-tasks-core"), "Hülle unvollständig: {all:?}");
    let banned = ["tauri", "wry", "tao", "webview2", "ureq", "reqwest", "hyper", "rustls", "native-tls", "tokio"];
    let hits: Vec<_> =
        all.iter().filter(|n| banned.iter().any(|b| n.as_str() == *b || n.starts_with(&format!("{b}-")))).collect();
    assert!(hits.is_empty(), "verbotene Crates im Baum des Store-Crates: {hits:?}");
}

#[test]
fn the_guard_would_see_tauri_in_the_shell() {
    // Gegenprobe: Dieselbe Prüfung findet Tauri in der Hülle, sie sagt also etwas aus.
    assert!(closure("daedyx-tasks").contains("tauri"));
}
