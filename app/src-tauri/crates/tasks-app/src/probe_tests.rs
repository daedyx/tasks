//! Installationsprobe in der laufenden App (nur Vorschau, über `probe.rs` eingebunden): erst sichern, dann starten.
use crate::test_support::tmp;

/// Review U2-R N-U2-1: erst den Entwurf sichern, dann starten (wie `install_update`); sonst startet nichts.
#[test]
fn probe_sichert_erst_und_startet_dann() {
    use crate::settings::Settings;
    use daedyx_updater::Status;
    use std::path::Path;
    let dir = tmp("u2-probe-entwurf");
    let setup = dir.join("setup.exe");
    std::fs::write(&setup, b"erfundenes Setup").unwrap();
    let calls = std::cell::RefCell::new(Vec::new());
    let starter = |p: &Path, s: &[u8; 32], a: &[&str]| {
        calls.borrow_mut().push((p.to_path_buf(), *s, a.join(" ")));
        Ok(())
    };
    let mut a = crate::test_support::app(&Settings::default());
    let t = a.tasks.items.first().cloned().unwrap();
    a.tasks.draft.id = t.id.clone();
    a.tasks.draft.title = t.title.clone();
    a.tasks.draft.notes = t.notes.clone();
    a.tasks.draft.date = "31.02.2026".into();
    a.run_update_probe_with(setup.clone(), &starter);
    assert!(calls.borrow().is_empty(), "nichts gestartet");
    assert_eq!(a.updater.status(), &Status::NotSaved);
    assert!(!a.bg.window.quit_wanted);

    let mut a = crate::test_support::app(&Settings::default());
    a.run_update_probe_with(setup.clone(), &starter);
    let sha = daedyx_sys::file_sha256(&setup).unwrap();
    assert_eq!(
        calls.borrow().as_slice(),
        &[(setup, sha, "/P /UPDATE /R".to_owned())]
    );
    assert_eq!(a.updater.status(), &Status::Installing);
    assert!(a.bg.window.quit_wanted);
}
