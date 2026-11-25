//! Startprobe des gebauten Programms (nur Windows, wie Desk M7b): Es lädt und endet mit `--start-probe` sofort, trägt
//! das Manifest für Common Controls 6 und das Tasks-Symbol.
#![cfg(windows)]
use daedyx_sys::{RunEnd, run_with_limit};
use std::path::Path;
use std::time::Duration;

const EXE: &str = env!("CARGO_BIN_EXE_daedyx-tasks-vorschau");

#[test]
fn programm_laedt_und_endet_bei_der_startprobe() {
    let end = run_with_limit(Path::new(EXE), &["--start-probe"], Duration::from_secs(15));
    assert_eq!(
        end,
        RunEnd::Exited(Some(0)),
        "Startprobe hängt oder scheitert (Manifest prüfen)"
    );
}

#[test]
fn programm_traegt_das_manifest_fuer_common_controls_6() {
    let bytes = std::fs::read(EXE).unwrap();
    let needle = b"Microsoft.Windows.Common-Controls";
    assert!(bytes.windows(needle.len()).any(|w| w == needle));
}

#[test]
fn programm_traegt_das_tasks_symbol() {
    let ico = include_bytes!("../../../icons/icon.ico");
    let exe = std::fs::read(EXE).unwrap();
    let mid = ico.len() / 2;
    let needle = &ico[mid..mid + 64];
    assert!(exe.windows(needle.len()).any(|w| w == needle));
}
