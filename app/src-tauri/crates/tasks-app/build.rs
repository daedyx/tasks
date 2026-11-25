//! Windows-Ausstattung des Vorschau-Programms über `daedyx-build`: Manifest mit Common Controls 6 (sonst kann der
//! Start hängen, Bericht M7b) und das Tasks-Symbol (`app/src-tauri/icons/icon.ico`). Geprüft von `tests/start_probe.rs`.
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    let icon = manifest_dir
        .join("..")
        .join("..")
        .join("icons")
        .join("icon.ico");
    let mut out = std::io::stdout();
    if daedyx_build::windows_app(&icon, "tasks-symbol.res") == daedyx_build::Symbol::Missing {
        let _ = writeln!(
            out,
            "cargo:warning=Programmsymbol nicht eingebettet (icons/icon.ico fehlt oder ist kaputt)"
        );
    }
    let _ = writeln!(out, "cargo:rerun-if-changed=build.rs");
}
