//! Wächtertests: Konfiguration und Installer-Bausteine dürfen nicht unbemerkt kippen.
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn conf() -> serde_json::Value {
    let text = std::fs::read_to_string(root().join("tauri.conf.json")).unwrap();
    serde_json::from_str(&text).unwrap()
}

fn manifest() -> String {
    std::fs::read_to_string(root().join("Cargo.toml")).unwrap()
}

#[test]
fn the_app_has_its_own_identity() {
    let c = conf();
    assert_eq!(c["productName"], "Daedyx Tasks");
    assert_eq!(c["identifier"], "de.daedyx.tasks");
    assert_eq!(c["mainBinaryName"], "daedyx-tasks");
    assert_eq!(c["app"]["windows"][0]["title"], "Daedyx Tasks");
    assert_eq!(crate::db::DB_FILE, "daedyx-tasks.db");
    assert!(crate::secrets::SERVICE.starts_with("daedyx-tasks"));
}

#[test]
fn the_dev_connector_is_optional_and_off_by_default() {
    let m = manifest();
    let line = m.lines().find(|l| l.starts_with("tauri-plugin-mcp-bridge")).expect("Eintrag fehlt");
    assert!(line.contains("optional = true"), "Konnektor muss optional sein: {line}");
    assert!(m.contains("dev-bridge = [\"dep:tauri-plugin-mcp-bridge\"]"));
    let default = m.lines().find(|l| l.starts_with("default")).unwrap_or("");
    assert!(!default.contains("dev-bridge"), "Konnektor darf nicht Standard sein");
}

#[test]
fn the_core_comes_from_git_with_a_fixed_tag() {
    let m = manifest();
    let line = m.lines().find(|l| l.starts_with("daedyx-tasks-core")).expect("Kern fehlt");
    assert!(line.contains("git = \"https://github.com/daedyx/core\""), "{line}");
    assert!(line.contains("tag = \""), "Kern muss auf einen Tag zeigen: {line}");
    assert!(!line.contains("path ="), "Pfad-Einbindung baut in der Pipeline nicht: {line}");
}

#[test]
fn installer_is_nsis_per_user_german_with_app_json_hook() {
    let c = conf();
    assert_eq!(c["bundle"]["targets"], serde_json::json!(["nsis"]));
    let nsis = &c["bundle"]["windows"]["nsis"];
    assert_eq!(nsis["installMode"], "currentUser");
    assert_eq!(nsis["languages"], serde_json::json!(["German"]));
    assert_eq!(nsis["installerHooks"], "windows/installer-hooks.nsh");
}

#[test]
fn the_hook_writes_and_removes_app_json_with_version_and_exe() {
    let hook = std::fs::read_to_string(root().join("windows/installer-hooks.nsh")).unwrap();
    assert!(hook.contains("!macro NSIS_HOOK_POSTINSTALL"));
    assert!(hook.contains("$INSTDIR\\app.json"));
    assert!(hook.contains("\"version\": \"${VERSION}\""));
    assert!(hook.contains("\"exe\": \"${MAINBINARYNAME}.exe\""));
    assert!(hook.contains("!macro NSIS_HOOK_POSTUNINSTALL"));
    assert!(hook.contains("Delete \"$INSTDIR\\app.json\""));
}

#[test]
fn updater_points_at_the_tasks_channel_and_is_consistent_with_its_key() {
    let c = conf();
    let u = &c["plugins"]["updater"];
    // Solange Björn den Update-Schlüssel nicht erzeugt hat, bleibt der Schlüssel leer, der Updater wird nicht
    // eingehängt und es gibt keine Update-Artefakte; mit Schlüssel müssen sie erzeugt werden.
    let has_key = !u["pubkey"].as_str().unwrap_or("").is_empty();
    assert_eq!(c["bundle"]["createUpdaterArtifacts"], has_key, "Update-Artefakte nur mit Schlüssel");
    assert_eq!(u["windows"]["installMode"], "passive");
    for e in u["endpoints"].as_array().expect("endpoints") {
        let e = e.as_str().unwrap();
        assert!(e.starts_with("https://"), "nur HTTPS: {e}");
        assert!(e.ends_with("/tasks/latest.json"), "falscher Kanal: {e}");
    }
    let caps = std::fs::read_to_string(root().join("capabilities/default.json")).unwrap();
    assert!(caps.contains("updater:default") && caps.contains("process:allow-restart"));
}

#[test]
fn dev_builds_use_their_own_data_folder_and_vault_service() {
    let dev: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root().join("tauri.dev.conf.json")).unwrap()).unwrap();
    let id = dev["identifier"].as_str().expect("Entwicklungs-Kennung fehlt");
    assert_eq!(id, "de.daedyx.tasks.dev");
    assert_ne!(id, conf()["identifier"].as_str().unwrap());
    assert_eq!(crate::secrets::SERVICE == "daedyx-tasks-dev", cfg!(feature = "dev-bridge"));
}
