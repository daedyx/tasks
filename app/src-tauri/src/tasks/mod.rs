//! Aufgaben: Rechenkern und Schema liegen im Crate `daedyx-tasks-core` (Repository `daedyx/core`), Grenzen und
//! Erinnerungstakt in `daedyx-tasks-store`. Hier bleibt nur, was Tauri braucht (`background`); die Re-Exporte halten
//! die Pfade `crate::tasks::…` stabil.
pub mod background;

// Ohne `reminders`: Der Kern-Durchgang rechnet ungeprüft; der Takt läuft nur über `daedyx_tasks_store::reminders`
// (Wächter `guard_core_tests` im Store-Crate).
pub use daedyx_tasks_core::{
    edit_alarms, edit_done, edit_lists, edit_sections, edit_tasks, errors, hlc, merge, meta, outbox, repeat, smart, stamp,
    store, types, zone, TaskError,
};
pub use daedyx_tasks_store::limits;
