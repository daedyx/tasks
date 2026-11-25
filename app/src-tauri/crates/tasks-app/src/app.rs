//! Zustand der App: Sprache und Texte, gewählte Seite, die Aufgaben-Oberfläche aus `daedyx-tasks-ui`.
use crate::page::Page;
use crate::settings::Settings;
use crate::texts::texts;
use daedyx_tasks_ui::{Mode, Sel, TaskStore, TasksUi};
use daedyx_ui::i18n::{Language, Texts};
use daedyx_ui::screenshot::Shot;
use eframe::egui::{Theme, Ui};

pub struct TasksApp {
    pub language: Language,
    pub texts: Texts,
    pub page: Page,
    pub tasks: TasksUi,
    dark: Option<bool>,
    started: bool,
    shot: Option<Shot>,
}

impl TasksApp {
    pub fn new(settings: &Settings, store: Option<TaskStore>) -> TasksApp {
        let mut app = TasksApp {
            language: settings.language,
            texts: texts(settings.language),
            page: Page::List,
            tasks: TasksUi::new(store),
            dark: settings.dark,
            started: false,
            shot: None,
        };
        app.open(settings.page);
        app
    }

    /// Foto nur vom App-Fenster (`DAEDYX_SCREENSHOT`), danach schließt sich das Fenster.
    pub fn with_shot(mut self, shot: Option<Shot>) -> TasksApp {
        self.shot = shot;
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.texts = texts(language);
    }

    /// Seite wählen. Das Board braucht eine Liste: ohne Auswahl nimmt es die erste.
    pub fn open(&mut self, page: Page) {
        self.page = page;
        match page {
            Page::List => self.tasks.mode = Mode::List,
            Page::Board => {
                if self.tasks.list_id().is_none()
                    && let Some(id) = self.tasks.lists.first().map(|l| l.id.clone())
                {
                    self.tasks.select(Sel::List(id));
                }
                self.tasks.mode = Mode::Board;
            }
            Page::Settings => {}
        }
    }

    /// Welche Seite die Leiste als gewählt zeigt. Der Umschalter in der Aufgaben-Oberfläche ändert die Ansicht
    /// auch; die Leiste folgt ihm.
    pub fn active(&self) -> Page {
        match self.page {
            Page::Settings => Page::Settings,
            _ if self.tasks.mode == Mode::Board && self.tasks.list_id().is_some() => Page::Board,
            _ => Page::List,
        }
    }

    pub fn frame(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx().clone();
        if !self.started {
            self.started = true;
            if let Some(dark) = self.dark {
                ctx.set_theme(if dark { Theme::Dark } else { Theme::Light });
            }
        }
        if let Some(shot) = &mut self.shot {
            shot.tick(&ctx);
        }
        crate::shell::show(ui, self);
    }
}

impl eframe::App for TasksApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.frame(ui);
    }
}
