//! Freier Platz auf dem Laufwerk eines Ordners (nur Windows; sonst keine Messung, die Kopie selbst meldet dann den Fehler).
use std::path::Path;

#[cfg(windows)]
pub fn available(dir: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetDiskFreeSpaceExW(dir: *const u16, free_to_caller: *mut u64, total: *mut u64, total_free: *mut u64) -> i32;
    }
    let wide: Vec<u16> = dir.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let mut free = 0u64;
    // SAFETY: `wide` ist nullterminiert und lebt über den Aufruf; die Zeiger zeigen auf lokale Werte.
    let ok = unsafe { GetDiskFreeSpaceExW(wide.as_ptr(), &mut free, std::ptr::null_mut(), std::ptr::null_mut()) };
    (ok != 0).then_some(free)
}

#[cfg(not(windows))]
pub fn available(_dir: &Path) -> Option<u64> {
    None
}
