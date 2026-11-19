//! Aufgaben: Rechenkern und Schema liegen im Crate `daedyx-tasks-core` (Repository `daedyx/core`).
//! Hier bleibt nur, was Tauri braucht (`background`); die Re-Exporte halten die Pfade `crate::tasks::…` stabil.
pub mod background;

pub use daedyx_tasks_core::{
    edit_done, edit_lists, edit_sections, edit_tasks, errors, hlc, merge, meta, outbox, reminders, repeat, smart, stamp,
    store, types, zone, TaskError,
};
