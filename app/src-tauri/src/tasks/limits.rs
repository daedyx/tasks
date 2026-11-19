//! Grenzen, die Tasks dem Kern vorgibt. Desk bleibt beim Kern-Standard (eine Ebene Unteraufgaben).
pub const TASKS_MAX_DEPTH: u32 = 6;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{edit_alarms, edit_tasks, edit_lists, TaskError};
    use rusqlite::Connection;
    use serde_json::{json, Map, Value};

    fn db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        daedyx_tasks_core::migrate(&conn).unwrap();
        conn
    }

    fn fields(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn tasks_allow_subtasks_down_to_the_limit_and_refuse_one_more() {
        let conn = db();
        let list = edit_lists::create_list(&conn, "Test", "", 1000).unwrap();
        let mut parent: Option<String> = None;
        for level in 0..=TASKS_MAX_DEPTH {
            let t = edit_tasks::create_task_depth(&conn, &list.id, parent.as_deref(), &fields(json!({"title": format!("Ebene {level}")})), 1000, TASKS_MAX_DEPTH)
                .unwrap_or_else(|e| panic!("Ebene {level} muss gehen: {e:?}"));
            parent = Some(t.id);
        }
        let too_deep = edit_tasks::create_task_depth(&conn, &list.id, parent.as_deref(), &fields(json!({"title": "zu tief"})), 1000, TASKS_MAX_DEPTH);
        assert!(matches!(too_deep, Err(TaskError::TooDeep)));
    }

    #[test]
    fn several_reminders_are_stored_sorted_and_deduplicated() {
        let conn = db();
        let list = edit_lists::create_list(&conn, "Test", "", 1000).unwrap();
        let t = edit_tasks::create_task(&conn, &list.id, None, &fields(json!({"title": "Termin"})), 1000).unwrap();
        edit_alarms::set_alarms(&conn, &t.id, &[1440, 10, 10, 0], 2000).unwrap();
        assert_eq!(edit_alarms::alarms(&conn, &t.id).unwrap(), vec![0, 10, 1440]);
    }
}
