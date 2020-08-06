use cursive::{Cursive, Printer};
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::vec::Vec2;
use std::path::Path;

use crate::gamemodel::{Action, GameModel, ObjectKind, Tile};

pub struct GameView {
    level_id: usize,
    model: GameModel,
    callback: Callback,
}

impl GameView {
    pub fn new<F>(level_id: usize, cb: F) -> Self
    where F: 'static + Fn(&mut Cursive) {
        GameView {
            level_id,
            model: GameModel::new(level_id),
            callback: Callback::from_fn(cb),
        }
    }

    pub fn load_level(&mut self, level_id: usize) {
        self.level_id = level_id;
        self.model = GameModel::new(level_id);
    }

    pub fn get_level(&self) -> usize {
        return self.level_id;
    }

    pub fn has_another_level(&self) -> bool {
        let path_s = &format!("levels/{}.txt", self.get_level() + 1);
        let path = Path::new(path_s);
        return path.exists();
    }
}

impl cursive::view::View for GameView {
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        self.model.get_level_extents()
    }

    fn draw(&self, printer: &Printer) {
        for (y, row) in self.model.tilemap.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                printer.print(Vec2::new(x, y), match tile {
                    Tile::Floor => " ",
                    Tile::Goal => ".",
                    Tile::Wall => "#",
                });
            }
        }

        for object in self.model.objects.iter() {
            printer.print(object.pos, match self.model.tilemap[object.pos.y][object.pos.x] {
                Tile::Floor => match object.kind {
                    ObjectKind::Player => "@",
                    ObjectKind::Box => "$",
                },
                Tile::Goal => match object.kind {
                    ObjectKind::Player => "+",
                    ObjectKind::Box => "*",
                },
                _ => panic!(),
            });
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) | Event::Char('w') => self.model.do_action(Action::Up),
            Event::Key(Key::Down) | Event::Char('s') => self.model.do_action(Action::Down),
            Event::Key(Key::Left) | Event::Char('a') => self.model.do_action(Action::Left),
            Event::Key(Key::Right) | Event::Char('d') => self.model.do_action(Action::Right),
            Event::Char('r') => self.model = GameModel::new(self.level_id),
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(if self.model.has_won() {Some(self.callback.clone())} else {None})
    }
}
