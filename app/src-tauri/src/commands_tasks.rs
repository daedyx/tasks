//! Tauri-Befehle für Aufgaben (dünne Hülle um `tasks/`). Fehler: `{code, message}` (siehe `TaskError::code`).
//! Nach jeder Änderung meldet der Kern `tasks://changed` {listId}.
use crate::commands::CommandError;
use crate::tasks::zone::zone;
use crate::tasks::edit_alarms;
use crate::tasks::edit_done::toggle_done;
use crate::tasks::limits::TASKS_MAX_DEPTH;
use crate::tasks::edit_lists as lists;
use crate::tasks::edit_sections as sections;
use crate::tasks::edit_tasks as edit;
use crate::tasks::smart::{self, View};
use crate::tasks::types::{Section, Task, TaskList};
use crate::tasks::{store, TaskError};
use chrono::{NaiveDate, TimeZone};
use rusqlite::Connection;
use serde::Serialize;
use serde_json::{Map, Value};
use tauri::{AppHandle, Emitter, State};

impl From<TaskError> for CommandError {
    fn from(e: TaskError) -> Self {
        CommandError { code: e.code(), message: e.to_string() }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Changed {
    list_id: Option<String>,
}

#[derive(Serialize)]
pub struct ViewCount {
    view: View,
    count: usize,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn today_in(tz: chrono_tz::Tz) -> NaiveDate {
    tz.timestamp_opt((now_ms() / 1000) as i64, 0).single().map(|d| d.date_naive()).unwrap_or_default()
}

fn with_conn<T>(state: &State<crate::AppState>, f: impl FnOnce(&Connection) -> Result<T, TaskError>) -> Result<T, CommandError> {
    let conn = state.db.lock().map_err(|_| CommandError { code: "internal", message: "Datenbank gesperrt".to_string() })?;
    f(&conn).map_err(Into::into)
}

fn changed(app: &AppHandle, list_id: Option<&str>) {
    let _ = app.emit("tasks://changed", Changed { list_id: list_id.map(str::to_string) });
}

#[tauri::command]
pub fn task_list_list(state: State<crate::AppState>) -> Result<Vec<TaskList>, CommandError> {
    with_conn(&state, store::lists)
}

#[tauri::command]
pub fn task_list_create(app: AppHandle, state: State<crate::AppState>, name: String, color: String) -> Result<TaskList, CommandError> {
    let l = with_conn(&state, |c| lists::create_list(c, &name, &color, now_ms()))?;
    changed(&app, Some(&l.id));
    Ok(l)
}

/// Ändert Name und/oder Farbe.
#[tauri::command]
pub fn task_list_update(app: AppHandle, state: State<crate::AppState>, id: String, name: Option<String>, color: Option<String>) -> Result<(), CommandError> {
    with_conn(&state, |c| {
        if let Some(n) = &name {
            lists::rename_list(c, &id, n, now_ms())?;
        }
        if let Some(col) = &color {
            lists::set_list_color(c, &id, col, now_ms())?;
        }
        Ok(())
    })?;
    changed(&app, Some(&id));
    Ok(())
}

#[tauri::command]
pub fn task_list_reorder(app: AppHandle, state: State<crate::AppState>, ids: Vec<String>) -> Result<(), CommandError> {
    with_conn(&state, |c| lists::reorder_lists(c, &ids, now_ms()))?;
    changed(&app, None);
    Ok(())
}

#[tauri::command]
pub fn task_list_delete(app: AppHandle, state: State<crate::AppState>, id: String) -> Result<(), CommandError> {
    with_conn(&state, |c| lists::delete_list(c, &id, now_ms()))?;
    changed(&app, Some(&id));
    Ok(())
}

/// Aufgaben einer Ansicht. `tz` = IANA-Zone der Anzeige (für „Heute“).
#[tauri::command]
pub fn task_query(state: State<crate::AppState>, view: View, list_id: Option<String>, tz: String) -> Result<Vec<Task>, CommandError> {
    let z = zone(&tz);
    with_conn(&state, |c| smart::query(c, view, list_id.as_deref(), today_in(z), z))
}

#[tauri::command]
pub fn task_counts(state: State<crate::AppState>, tz: String) -> Result<Vec<ViewCount>, CommandError> {
    let z = zone(&tz);
    let counts = with_conn(&state, |c| smart::counts(c, today_in(z), z))?;
    Ok(counts.into_iter().map(|(view, count)| ViewCount { view, count }).collect())
}

#[tauri::command]
pub fn task_search(state: State<crate::AppState>, query: String) -> Result<Vec<Task>, CommandError> {
    with_conn(&state, |c| smart::search(c, &query, 50))
}

#[tauri::command]
pub fn task_create(app: AppHandle, state: State<crate::AppState>, list_id: String, parent_id: Option<String>, fields: Map<String, Value>) -> Result<Task, CommandError> {
    let t = with_conn(&state, |c| edit::create_task_depth(c, &list_id, parent_id.as_deref(), &fields, now_ms(), TASKS_MAX_DEPTH))?;
    changed(&app, Some(&list_id));
    Ok(t)
}

#[tauri::command]
pub fn task_update(app: AppHandle, state: State<crate::AppState>, id: String, patch: Map<String, Value>) -> Result<Task, CommandError> {
    let t = with_conn(&state, |c| edit::update_task(c, &id, &patch, now_ms()))?;
    changed(&app, Some(&t.list_id));
    Ok(t)
}

#[tauri::command]
pub fn task_toggle_done(app: AppHandle, state: State<crate::AppState>, id: String, tz: String) -> Result<Task, CommandError> {
    let t = with_conn(&state, |c| toggle_done(c, &id, now_ms(), zone(&tz)))?;
    changed(&app, Some(&t.list_id));
    Ok(t)
}

#[tauri::command]
pub fn task_move(app: AppHandle, state: State<crate::AppState>, id: String, list_id: String, parent_id: Option<String>, section_id: Option<String>, position: f64) -> Result<Task, CommandError> {
    let old = with_conn(&state, |c| store::task(c, &id))?.list_id;
    let t = with_conn(&state, |c| edit::move_task_depth(c, &id, &list_id, parent_id.as_deref(), section_id.as_deref(), position, now_ms(), TASKS_MAX_DEPTH))?;
    changed(&app, Some(&old));
    changed(&app, Some(&t.list_id));
    Ok(t)
}

#[tauri::command]
pub fn task_delete(app: AppHandle, state: State<crate::AppState>, id: String) -> Result<(), CommandError> {
    let list_id = with_conn(&state, |c| store::task(c, &id))?.list_id;
    with_conn(&state, |c| edit::delete_task(c, &id, now_ms()))?;
    changed(&app, Some(&list_id));
    Ok(())
}

#[tauri::command]
pub fn task_section_list(state: State<crate::AppState>, list_id: String) -> Result<Vec<Section>, CommandError> {
    with_conn(&state, |c| store::sections(c, &list_id))
}

#[tauri::command]
pub fn task_section_create(app: AppHandle, state: State<crate::AppState>, list_id: String, name: String) -> Result<Section, CommandError> {
    let s = with_conn(&state, |c| sections::create_section(c, &list_id, &name, now_ms()))?;
    changed(&app, Some(&list_id));
    Ok(s)
}

#[tauri::command]
pub fn task_section_rename(app: AppHandle, state: State<crate::AppState>, id: String, name: String) -> Result<(), CommandError> {
    let list_id = with_conn(&state, |c| store::section(c, &id))?.list_id;
    with_conn(&state, |c| sections::rename_section(c, &id, &name, now_ms()))?;
    changed(&app, Some(&list_id));
    Ok(())
}

#[tauri::command]
pub fn task_section_reorder(app: AppHandle, state: State<crate::AppState>, list_id: String, ids: Vec<String>) -> Result<(), CommandError> {
    with_conn(&state, |c| sections::reorder_sections(c, &list_id, &ids, now_ms()))?;
    changed(&app, Some(&list_id));
    Ok(())
}

#[tauri::command]
pub fn task_section_delete(app: AppHandle, state: State<crate::AppState>, id: String) -> Result<(), CommandError> {
    let list_id = with_conn(&state, |c| store::section(c, &id))?.list_id;
    with_conn(&state, |c| sections::delete_section(c, &id, now_ms()))?;
    changed(&app, Some(&list_id));
    Ok(())
}

/// Erinnerungen der Aufgabe (Minuten vor der Fälligkeit, aufsteigend).
#[tauri::command]
pub fn task_alarms_get(state: State<crate::AppState>, id: String) -> Result<Vec<i64>, CommandError> {
    with_conn(&state, |c| edit_alarms::alarms(c, &id))
}

/// Ersetzt die Erinnerungen der Aufgabe (höchstens zehn, doppelte fallen weg).
#[tauri::command]
pub fn task_alarms_set(app: AppHandle, state: State<crate::AppState>, id: String, minutes: Vec<i64>) -> Result<Vec<i64>, CommandError> {
    let list_id = with_conn(&state, |c| store::task(c, &id))?.list_id;
    let out = with_conn(&state, |c| {
        edit_alarms::set_alarms(c, &id, &minutes, now_ms())?;
        edit_alarms::alarms(c, &id)
    })?;
    changed(&app, Some(&list_id));
    Ok(out)
}
