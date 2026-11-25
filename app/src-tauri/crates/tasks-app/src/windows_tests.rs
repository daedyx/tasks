//! Windows (T4, wie Desk M7b): Autostart nur unter einem Testschlüssel `HKCU\Software\DaedyxTasksVorschauTest\<pid>`
//! (nie unter `...\Run`, danach gelöscht); Einzelinstanz über die Pipe mit dem Namensanfang der Vorschau und einem
//! Test-Bereich je Prozess (stört keine laufende Vorschau). Nachrichten: nur `SHOW`/`QUIT` wirken.
use crate::ids::{PIPE_PREFIX, RUN_VALUE};
use daedyx_background::autostart::{AutostartStore, RegistryStore, State};
use daedyx_background::ids::pipe_name;
use daedyx_background::instance::{self, Role};
use daedyx_background::{Event, Sink};
use std::time::Duration;

/// Räumt den Testschlüssel auch dann ab, wenn eine Prüfung mittendrin fehlschlägt (Review T4, N4-3).
struct TestKey(String);

impl Drop for TestKey {
    fn drop(&mut self) {
        let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(&self.0);
        let _ = hkcu.delete_subkey(r"Software\DaedyxTasksVorschauTest");
    }
}

#[test]
fn autostart_ein_aus_und_blockiert_unter_testschluessel() {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, REG_BINARY};
    let base = format!(r"Software\DaedyxTasksVorschauTest\{}", std::process::id());
    let guard = TestKey(base.clone());
    let mut store = RegistryStore {
        run_key: format!(r"{base}\Run"),
        approved_key: format!(r"{base}\Approved"),
        value: RUN_VALUE.into(),
        command: r#""/x/daedyx-tasks-vorschau.exe" --hidden"#.into(),
    };
    assert_eq!(store.state().unwrap(), State::Off);
    store.set(true).unwrap();
    assert_eq!(store.state().unwrap(), State::On);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (ap, _) = hkcu.create_subkey(&store.approved_key).unwrap();
    let blocked = winreg::RegValue {
        bytes: vec![3, 0, 0, 0].into(),
        vtype: REG_BINARY,
    };
    ap.set_raw_value(RUN_VALUE, &blocked).unwrap();
    assert_eq!(store.state().unwrap(), State::Blocked);
    store.set(false).unwrap();
    assert_eq!(store.state().unwrap(), State::Off);
    drop(guard);
    assert!(hkcu.open_subkey(&base).is_err(), "keine Reste");
}

fn events(rx: &std::sync::mpsc::Receiver<Event>, wait: Duration) -> Vec<Event> {
    let mut out = Vec::new();
    while let Ok(e) = rx.recv_timeout(wait) {
        out.push(e);
    }
    out
}

#[test]
fn zweiter_start_holt_das_fenster_und_unsinn_wirkt_nicht() {
    let rt = daedyx_background::start::runtime().unwrap();
    let name = pipe_name(PIPE_PREFIX, &format!("test-{}", std::process::id()));
    assert!(name.contains("daedyx-tasks-vorschau-instanz-"));
    let Role::Primary(listener) = rt.block_on(async { instance::acquire(&name) }).unwrap() else {
        panic!("erster Start muss die Pipe bekommen");
    };
    assert!(matches!(
        rt.block_on(async { instance::acquire(&name) }).unwrap(),
        Role::Secondary
    ));
    let (sink, rx) = Sink::new();
    rt.spawn(listener.serve(sink));
    for junk in [
        &b"SHO"[..],
        b"show",
        b"SHOWSHOWSHOWSHOWSHOW",
        b"",
        b"QUIT\0",
    ] {
        rt.block_on(instance::notify_primary(&name, junk)).unwrap();
    }
    assert!(
        events(&rx, Duration::from_millis(700)).is_empty(),
        "Unsinn wirkt nicht"
    );
    // Jede Nachricht ist eine eigene Verbindung; deren Reihenfolge ist nicht festgelegt. Deshalb erst SHOW abwarten,
    // dann QUIT schicken (Korrekturrunde T5: vorher kippte der Test unter Last).
    rt.block_on(instance::notify_primary(&name, instance::SHOW))
        .unwrap();
    assert_eq!(events(&rx, Duration::from_millis(700)), vec![Event::Show]);
    rt.block_on(instance::notify_primary(&name, instance::QUIT))
        .unwrap();
    assert_eq!(events(&rx, Duration::from_millis(700)), vec![Event::Quit]);
    rt.shutdown_timeout(Duration::from_secs(1));
}
