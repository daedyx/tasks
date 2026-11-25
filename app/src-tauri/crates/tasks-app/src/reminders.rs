//! Takt der Erinnerungen (T4): Jeder Durchgang ruft den geprüften Durchgang des Stores (`reminders::tick_once`) auf
//! derselben Speicher-Datenbank, die die Oberfläche zeigt. Der Store merkt sich jede gemeldete Erinnerung, so kommt
//! jede genau einmal, auch bei bis zu zehn Erinnerungen je Aufgabe. Kommen in einem Durchgang mehrere Erinnerungen
//! derselben Aufgabe (nachgeholt nach Ruhezustand), zeigt nur die späteste eine Meldung, die anderen gelten still als
//! gemeldet (Review T4, N4-1). Der Text nennt die echte Restzeit („Fällig in …“, „Jetzt fällig“, „Seit … fällig“)
//! und ist ohne Titel, solange der Schalter aus ist (T-F6).
use daedyx_background::ticker::{Jobs, Tick};
use daedyx_background::{Notice, NoticeKind};
use daedyx_tasks_store::reminders::{Reminder, tick_once, unix_now};
use daedyx_tasks_ui::TaskDb;
use rusqlite::Connection;
use std::sync::{Arc, Mutex, MutexGuard};

/// Eine Speicher-Datenbank für Oberfläche und Takt.
pub type SharedConn = Arc<Mutex<Connection>>;

/// Zugang der Oberfläche zur geteilten Datenbank (eine vergiftete Sperre wird übernommen, wie in `daedyx-tasks-ui`).
pub struct SharedDb(pub SharedConn);

impl TaskDb for SharedDb {
    fn lock(&self) -> MutexGuard<'_, Connection> {
        TaskDb::lock(self.0.as_ref())
    }
}

/// Uhr des Takts in Unix-Sekunden (Programm: Systemzeit, Tests: fest).
pub type Now = Box<dyn Fn() -> i64 + Send + Sync>;

pub struct ReminderJobs {
    db: SharedConn,
    zone: String,
    now: Now,
}

impl ReminderJobs {
    pub fn new(db: SharedConn, zone: &str) -> ReminderJobs {
        ReminderJobs::with_clock(db, zone, Box::new(unix_now))
    }

    pub fn with_clock(db: SharedConn, zone: &str, now: Now) -> ReminderJobs {
        ReminderJobs {
            db,
            zone: zone.to_owned(),
            now,
        }
    }
}

/// Ganze Minuten von `now` bis zur Fälligkeit (Zeitpunkt der Erinnerung plus Vorlauf), auf die nächste Minute
/// gerundet; negativ, wenn schon fällig. Rechnet geprüft; ein unsinniger Wert ergibt „Jetzt fällig“.
pub fn minutes_until(r: &Reminder, now: i64) -> i64 {
    r.minutes_before
        .checked_mul(60)
        .and_then(|lead| r.fire_at.checked_add(lead))
        .and_then(|due| due.checked_sub(now))
        .and_then(|secs| secs.checked_add(30))
        .map_or(0, |secs| secs.div_euclid(60))
}

impl Jobs for ReminderJobs {
    fn run(&self) -> Tick {
        let now = (self.now)();
        // Je Aufgabe nur die späteste Erinnerung dieses Durchgangs; der Store merkt sich trotzdem alle als gemeldet.
        let mut latest: Vec<Reminder> = Vec::new();
        tick_once(&self.db, now, &self.zone, |r| {
            match latest.iter_mut().find(|l| l.task_id == r.task_id) {
                Some(l) if r.fire_at >= l.fire_at => *l = r.clone(),
                Some(_) => {}
                None => latest.push(r.clone()),
            }
        });
        let notices = latest
            .iter()
            .map(|r| Notice {
                kind: NoticeKind::TaskLead {
                    minutes_until: minutes_until(r, now),
                },
                title: r.title.clone(),
                time: String::new(),
            })
            .collect();
        Tick {
            notices,
            sync_due: 0,
        }
    }
}
