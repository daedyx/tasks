//! Geprüfter Erinnerungs-Durchgang (Review T0, M-1). Gleiche Regeln wie `daedyx_tasks_core::reminders::{pending,
//! tick}` (Kern-Tag `tasks-core-v0.2.1`), aber jede Rechnung geprüft: Der Kern rechnet `due - minuten * 60` ungeprüft,
//! und unsinnige Werte aus der Datenbank (veränderte Datei, später ein Abgleich) brächten das Programm mit
//! `overflow-checks` und `panic = "abort"` bei jedem Start zum Absturz. Ungültige Erinnerungen werden übersprungen.
//! Ein Test hält die Gleichheit mit dem Kern für gültige Daten fest; mit einem Kern-Tag, der selbst prüft, kann diese
//! Datei wieder entfallen.
use chrono::TimeZone;
use daedyx_tasks_core::reminders::{mark_fired, DueReminder, CATCH_UP_SECS};
use daedyx_tasks_core::smart::due_day;
use daedyx_tasks_core::store::{task_from_row, TASK_COLS};
use daedyx_tasks_core::types::Task;
use daedyx_tasks_core::zone::zone;
use daedyx_tasks_core::{edit_alarms, TaskError};
use rusqlite::Connection;

/// Letzte Sekunde, die als Uhrzeit oder Fälligkeit gilt (9999-12-31 23:59:59 UTC); die erste ist 1970-01-01.
pub const MAX_SECS: i64 = 253_402_300_799;
/// Größter Vorlauf einer Erinnerung in Minuten (ein Jahr), wie die Eingabeprüfung des Kerns (`set_alarms`).
pub const MAX_MINUTES: i64 = 525_600;
const TWO_DAYS: i64 = 2 * 86_400;

/// Bezugszeit der Aufgabe: mit Uhrzeit die Fälligkeit, sonst 9:00 Ortszeit des Tages. `None` = keine oder unsinnig.
fn base_secs(t: &Task, tz: chrono_tz::Tz) -> Option<i64> {
    let secs = if t.due_has_time {
        t.due.as_deref()?.parse::<i64>().ok()?
    } else {
        let local = due_day(t, tz)?.and_hms_opt(9, 0, 0)?;
        tz.from_local_datetime(&local).earliest()?.timestamp()
    };
    (0..=MAX_SECS).contains(&secs).then_some(secs)
}

/// Offene Aufgaben mit Erinnerung, deren Zeitpunkt in `[from, to)` liegt und die noch nicht gemeldet wurden.
pub fn pending_checked(conn: &Connection, from: i64, to: i64, tz_name: &str) -> Result<Vec<DueReminder>, TaskError> {
    let tz = zone(tz_name);
    let sql = format!(
        "SELECT {TASK_COLS} FROM tasks WHERE deleted_at IS NULL AND done_at IS NULL AND due IS NOT NULL \
         AND (remind_minutes IS NOT NULL OR EXISTS (SELECT 1 FROM task_alarms a WHERE a.task_id = tasks.id AND a.deleted_at IS NULL)) \
         AND list_id IN (SELECT id FROM task_lists WHERE deleted_at IS NULL)"
    );
    let tasks = conn.prepare(&sql)?.query_map([], task_from_row)?.collect::<Result<Vec<_>, _>>()?;
    let mut fired = conn.prepare("SELECT 1 FROM task_alarms_fired WHERE task_id = ?1 AND fire_at = ?2")?;
    let mut out = Vec::new();
    for t in tasks {
        let Some(base) = base_secs(&t, tz) else { continue };
        let mut mins = edit_alarms::alarms(conn, &t.id)?;
        if mins.is_empty() {
            mins.extend(t.remind_minutes);
        }
        for min in mins {
            if !(0..=MAX_MINUTES).contains(&min) {
                continue;
            }
            let Some(fire_at) = min.checked_mul(60).and_then(|s| base.checked_sub(s)) else { continue };
            if fire_at < from || fire_at >= to || fired.exists((&t.id, fire_at))? {
                continue;
            }
            out.push(DueReminder {
                task_id: t.id.clone(),
                title: t.title.clone(),
                fire_at,
                minutes_before: min,
                due_has_time: t.due_has_time,
            });
        }
    }
    out.sort_by_key(|r| r.fire_at);
    Ok(out)
}

/// Ein Durchgang wie `daedyx_tasks_core::reminders::tick`, mit geprüften Rechnungen.
pub fn tick_checked(
    conn: &Connection,
    now: i64,
    tz_name: &str,
    mut fire: impl FnMut(&DueReminder),
) -> Result<usize, TaskError> {
    conn.execute("DELETE FROM task_alarms_fired WHERE fire_at < ?1", [now.saturating_sub(TWO_DAYS)])?;
    let due = pending_checked(conn, now.saturating_sub(CATCH_UP_SECS), now.saturating_add(1), tz_name)?;
    for r in &due {
        fire(r);
        mark_fired(conn, r)?;
    }
    Ok(due.len())
}
