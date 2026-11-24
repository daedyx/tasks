//! Wächter (plan-tasks-rust T0, 5.1): Das Store-Crate hängt weder an Tauri noch am Netz, geprüft gegen eine
//! Erlaubnisliste. Geprüft wird die volle Abhängigkeits-Hülle laut `Cargo.lock` (alle Zielsysteme, auch
//! Bau-Abhängigkeiten), ohne einen Prozess zu starten.
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

/// Erlaubnisliste (geschlossene Welt, Review T0 N-5): genau diese Crates bilden heute die Hülle des Store-Crates laut
/// `Cargo.lock` (alle Zielsysteme, Bau- und Test-Abhängigkeiten eingeschlossen). Kein Netz, keine Oberfläche. Jede neue
/// Abhängigkeit, auch eine mittelbare, macht den Test rot und braucht einen Eintrag hier (und in der Prüfung nach
/// SICHERHEIT 4.1); eine weggefallene ebenso, damit die Liste wahr bleibt.
const ALLOWED: [&str; 92] = [
    "android_system_properties",
    "autocfg",
    "bitflags",
    "bumpalo",
    "byteorder",
    "cc",
    "cfg-if",
    "chrono",
    "chrono-tz",
    "core-foundation",
    "core-foundation-sys",
    "daedyx-tasks-core",
    "daedyx-tasks-store",
    "fallible-iterator",
    "fallible-streaming-iterator",
    "fastrand",
    "find-msvc-tools",
    "foldhash",
    "futures-core",
    "futures-io",
    "futures-macro",
    "futures-sink",
    "futures-task",
    "futures-util",
    "getrandom",
    "hashbrown",
    "hashlink",
    "iana-time-zone",
    "iana-time-zone-haiku",
    "itoa",
    "js-sys",
    "keyring",
    "libc",
    "libsqlite3-sys",
    "log",
    "memchr",
    "num-traits",
    "openssl-src",
    "openssl-sys",
    "phf",
    "phf_generator",
    "phf_macros",
    "phf_shared",
    "pin-project-lite",
    "pkg-config",
    "proc-macro2",
    "quote",
    "r-efi",
    "rand_core",
    "rusqlite",
    "rustversion",
    "security-framework",
    "security-framework-sys",
    "serde",
    "serde_core",
    "serde_derive",
    "serde_json",
    "shlex",
    "siphasher",
    "slab",
    "smallvec",
    "syn",
    "thiserror",
    "thiserror-impl",
    "unicode-ident",
    "vcpkg",
    "wasi",
    "wasip2",
    "wasm-bindgen",
    "wasm-bindgen-macro",
    "wasm-bindgen-macro-support",
    "wasm-bindgen-shared",
    "windows-core",
    "windows-implement",
    "windows-interface",
    "windows-link",
    "windows-result",
    "windows-strings",
    "windows-sys",
    "windows-targets",
    "windows_aarch64_gnullvm",
    "windows_aarch64_msvc",
    "windows_i686_gnu",
    "windows_i686_gnullvm",
    "windows_i686_msvc",
    "windows_x86_64_gnu",
    "windows_x86_64_gnullvm",
    "windows_x86_64_msvc",
    "wit-bindgen",
    "zeroize",
    "zeroize_derive",
    "zmij",
];

#[test]
fn the_store_has_no_tauri_and_no_network_in_its_tree() {
    let all = closure("daedyx-tasks-store");
    let allowed: BTreeSet<String> = ALLOWED.iter().map(|s| s.to_string()).collect();
    let unknown: Vec<_> = all.difference(&allowed).collect();
    let gone: Vec<_> = allowed.difference(&all).collect();
    assert!(unknown.is_empty(), "nicht erlaubte Crates im Baum des Store-Crates: {unknown:?}");
    assert!(gone.is_empty(), "Erlaubnisliste enthält Crates, die nicht mehr im Baum sind: {gone:?}");
}

#[test]
fn the_guard_would_see_tauri_in_the_shell() {
    // Gegenprobe: Dieselbe Prüfung findet in der Hülle Tauri und Netz-Crates außerhalb der Liste, sie sagt also etwas aus.
    let shell = closure("daedyx-tasks");
    for name in ["tauri", "wry", "reqwest"] {
        assert!(shell.contains(name) && !ALLOWED.contains(&name), "{name}");
    }
}
