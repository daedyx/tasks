//! Meldungsfenster für Fehler vor dem ersten Fenster. Ein eigenes Betriebssystem-Fenster, weil die Oberfläche dann noch nicht läuft.

/// Zeigt Titel und Text und kehrt erst zurück, wenn die Person es geschlossen hat.
#[cfg(windows)]
pub fn show_error(title: &str, message: &str) {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MessageBoxW(hwnd: *mut core::ffi::c_void, text: *const u16, caption: *const u16, kind: u32) -> i32;
    }
    // MB_ICONERROR | MB_SETFOREGROUND: das Fenster kommt in den Vordergrund, auch beim Autostart nach der Anmeldung.
    const MB_OK_ICONERROR: u32 = 0x0000_0010 | 0x0001_0000;
    let wide = |s: &str| -> Vec<u16> { std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect() };
    let (t, m) = (wide(title), wide(message));
    // SAFETY: beide Puffer sind nullterminiert und leben über den Aufruf; ohne Elternfenster (null) ist erlaubt.
    unsafe { MessageBoxW(std::ptr::null_mut(), m.as_ptr(), t.as_ptr(), MB_OK_ICONERROR) };
}

#[cfg(not(windows))]
pub fn show_error(title: &str, message: &str) {
    eprintln!("{title}: {message}");
}
