//! Geprüfter Erinnerungs-Durchgang (Review T0, M-1). Gleiche Regeln wie `daedyx_tasks_core::reminders::{pending,
//! tick}` (geschrieben gegen Kern-Tag `tasks-core-v0.2.1`), aber jede Rechnung geprüft: Der Kern rechnete
//! `due - minuten * 60` ungeprüft, und unsinnige Werte aus der Datenbank (veränderte Datei, später ein Abgleich)
//! brächten das Programm mit `overflow-checks` und `panic = "abort"` bei jedem Start zum Absturz. Ungültige
//! Erinnerungen werden übersprungen.
//! Ein Test hält die Gleichheit mit dem Kern für gültige Daten fest. Ab `tasks-core-v0.2.2` prüft der Kern selbst
//! (Tasks T1a bindet ihn ein); der Rückbau dieser Datei ist ein eigener Schritt, T1a zieht nur um.
use chrono::TimeZone;
use daedyx_tasks_core::reminders::{mark_fired, DueReminder, CATCH_UP_SECS};
use daedyx_tasks_core::smart::due_day;
use daedyx_tasks_core::store::{task_from_row, TASK_COLS};
use daedyx_tasks_core::types::Task;
use daedyx_tasks_core::zone::zone;
use daedyx_tasks_core::TaskError;
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

/// Vorlauf und Zeitpunkt jeder gültigen Erinnerung der Aufgabe. Wie `edit_alarms::alarms`, aber je Zeile: kaputte
/// Zeilen zählen als vorhanden (kein Rückfall auf das alte Einzelfeld), werden aber nicht gemeldet.
fn fire_times(
    t: &Task,
    tz: chrono_tz::Tz,
    alarm_rows: &mut rusqlite::Statement<'_>,
) -> rusqlite::Result<Vec<(i64, i64)>> {
    let Some(base) = base_secs(t, tz) else { return Ok(Vec::new()) };
    let rows: Vec<rusqlite::Result<i64>> = alarm_rows.query_map([&t.id], |r| r.get(0))?.collect();
    let has_rows = !rows.is_empty();
    let mut mins: Vec<i64> = rows.into_iter().filter_map(Result::ok).collect();
    if !has_rows {
        mins.extend(t.remind_minutes);
    }
    Ok(mins
        .into_iter()
        .filter(|min| (0..=MAX_MINUTES).contains(min))
        .filter_map(|min| Some((min, min.checked_mul(60).and_then(|s| base.checked_sub(s))?)))
        .collect())
}

/// Offene Aufgaben mit Erinnerung, deren Zeitpunkt in `[from, to)` liegt und die noch nicht gemeldet wurden.
pub fn pending_checked(conn: &Connection, from: i64, to: i64, tz_name: &str) -> Result<Vec<DueReminder>, TaskError> {
    let tz = zone(tz_name);
    let sql = format!(
        "SELECT {TASK_COLS} FROM tasks WHERE deleted_at IS NULL AND done_at IS NULL AND due IS NOT NULL \
         AND (remind_minutes IS NOT NULL OR EXISTS (SELECT 1 FROM task_alarms a WHERE a.task_id = tasks.id AND a.deleted_at IS NULL)) \
         AND list_id IN (SELECT id FROM task_lists WHERE deleted_at IS NULL)"
    );
    // Zeilen einzeln: Eine Zeile mit falschem Datentyp (SQLite erlaubt ihn in jeder Spalte) fällt allein weg und
    // schaltet nicht alle Erinnerungen ab (Review T0, N-7).
    let tasks: Vec<Task> = conn.prepare(&sql)?.query_map([], task_from_row)?.filter_map(Result::ok).collect();
    let mut alarm_rows = conn.prepare(
        "SELECT minutes_before FROM task_alarms WHERE task_id = ?1 AND deleted_at IS NULL ORDER BY minutes_before",
    )?;
    let mut fired = conn.prepare("SELECT 1 FROM task_alarms_fired WHERE task_id = ?1 AND fire_at = ?2")?;
    let mut out = Vec::new();
    for t in tasks {
        for (min, fire_at) in fire_times(&t, tz, &mut alarm_rows)? {
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

/// Räumt Merker gemeldeter Erinnerungen auf (Review T4, N4-2): nur solche, die älter als zwei Tage sind **und** zu
/// keiner Erinnerung mehr gehören, die noch kommen könnte (Aufgabe erledigt, gelöscht, ohne Fälligkeit, Erinnerung
/// oder Fälligkeit geändert). Merker einer noch gültigen Erinnerung bleiben, auch lange nach der Fälligkeit plus
/// Nachholfenster; je offener Aufgabe sind das höchstens so viele wie Erinnerungen. So kann eine Uhr, die weit vor
/// und wieder zurück springt, keine Erinnerung zweimal auslösen.
fn prune_fired(conn: &Connection, now: i64, tz_name: &str) -> Result<(), TaskError> {
    let old: Vec<(String, i64)> = conn
        .prepare("SELECT task_id, fire_at FROM task_alarms_fired WHERE fire_at < ?1")?
        .query_map([now.saturating_sub(TWO_DAYS)], |r| Ok((r.get(0)?, r.get(1)?)))?
        .filter_map(Result::ok)
        .collect();
    if old.is_empty() {
        return Ok(());
    }
    let tz = zone(tz_name);
    let mut task = conn.prepare(&format!(
        "SELECT {TASK_COLS} FROM tasks WHERE id = ?1 AND deleted_at IS NULL AND done_at IS NULL AND due IS NOT NULL          AND list_id IN (SELECT id FROM task_lists WHERE deleted_at IS NULL)"
    ))?;
    let mut alarm_rows = conn.prepare(
        "SELECT minutes_before FROM task_alarms WHERE task_id = ?1 AND deleted_at IS NULL ORDER BY minutes_before",
    )?;
    let mut delete = conn.prepare("DELETE FROM task_alarms_fired WHERE task_id = ?1 AND fire_at = ?2")?;
    for (id, at) in old {
        let open = task.query_map([&id], task_from_row)?.next().and_then(Result::ok);
        let live = match open {
            Some(t) => fire_times(&t, tz, &mut alarm_rows)?.iter().any(|(_, f)| *f == at),
            None => false,
        };
        if !live {
            delete.execute((&id, at))?;
        }
    }
    Ok(())
}

/// Ein Durchgang wie `daedyx_tasks_core::reminders::tick`, mit geprüften Rechnungen.
pub fn tick_checked(
    conn: &Connection,
    now: i64,
    tz_name: &str,
    mut fire: impl FnMut(&DueReminder),
) -> Result<usize, TaskError> {
    prune_fired(conn, now, tz_name)?;
    let due = pending_checked(conn, now.saturating_sub(CATCH_UP_SECS), now.saturating_add(1), tz_name)?;
    for r in &due {
        fire(r);
        mark_fired(conn, r)?;
    }
    Ok(due.len())
}
