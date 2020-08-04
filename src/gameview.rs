use cursive::{Cursive, Printer};
use cursive::event::{Callback, Event, EventResult};
use cursive::vec::Vec2;

use std::path::Path;

pub struct GameView {
    level_id: usize,
    callback: Callback,
}

impl GameView {
    pub fn new<F>(level_id: usize, cb: F) -> Self
    where F: 'static + Fn(&mut Cursive) {
        GameView {
            level_id,
            callback: Callback::from_fn(cb),
        }
    }

    pub fn load_level(&mut self, level_id: usize) {
        self.level_id = level_id;
    }

    pub fn get_level(&self) -> usize {
        return self.level_id;
    }

    pub fn has_another_level(&self) -> bool {
        let s = &format!("levels/{}.txt", self.get_level() + 1);
        return Path::new(s).exists();
    }
}

impl cursive::view::View for GameView {
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        return Vec2::new(self.level_id, 1);
    }

    fn draw(&self, printer: &Printer) {
        printer.print_hline(Vec2::new(0, 0), self.level_id, "~");
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('y') => EventResult::Consumed(Some(self.callback.clone())),
            _ => EventResult::Ignored,
        }
    }
}
