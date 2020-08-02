use cursive::Cursive;
use cursive::Printer;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::vec::Vec2;
use cursive::views::Dialog;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, PartialEq)]
enum SokobanTile {
    Empty,
    Goal,
    Wall,
}

impl SokobanTile {
    fn issolid(&self) -> bool {
        match self {
            SokobanTile::Wall => true,
            _ => false,
        }
    }
}

enum SokobanTileRaw {
    Wall,
    Player,
    GoalPlayer,
    Box,
    GoalBox,
    Goal,
    Empty,
}

struct SokobanTilemap {
    width: usize,
    tiles: Vec<SokobanTile>,
}

#[derive(PartialEq)]
enum SokobanObjectType {
    Player,
    Box,
}

#[derive(Copy, Clone)]
enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

struct SokobanObject {
    r#type: SokobanObjectType,
    pos: Vec2,
}

struct SokobanGame {
    tilemap: SokobanTilemap,
    objects: Vec<SokobanObject>,
}

impl SokobanGame {
    fn new(width: usize, levelstring: Vec<SokobanTileRaw>) -> SokobanGame {
        assert!(levelstring.len() % width == 0);

        let mut game = SokobanGame {
            tilemap: SokobanTilemap {
                width,
                tiles: vec![SokobanTile::Empty; levelstring.len()],
            },
            objects: Vec::new(),
        };

        for (i, c) in levelstring.iter().enumerate() {
            game.tilemap.tiles[i] = match c {
                SokobanTileRaw::Empty |
                SokobanTileRaw::Box |
                SokobanTileRaw::Player => SokobanTile::Empty,
                SokobanTileRaw::Goal |
                SokobanTileRaw::GoalBox |
                SokobanTileRaw::GoalPlayer => SokobanTile::Goal,
                SokobanTileRaw::Wall => SokobanTile::Wall,
            };

            match c {
                SokobanTileRaw::Player |
                SokobanTileRaw::GoalPlayer => {
                    game.objects.push(SokobanObject {
                        r#type: SokobanObjectType::Player,
                        pos: Vec2::new(i % game.tilemap.width, i / game.tilemap.width),
                    });
                },
                SokobanTileRaw::Box |
                SokobanTileRaw::GoalBox => {
                    game.objects.push(SokobanObject {
                        r#type: SokobanObjectType::Box,
                        pos: Vec2::new(i % game.tilemap.width, i / game.tilemap.width),
                    });
                },
                _ => (),
            };
        }

        game
    }

    fn has_won(&self) -> bool {
        for object in &self.objects {
            if object.r#type != SokobanObjectType::Box {continue;}
            let linearpos = object.pos.x + object.pos.y * self.tilemap.width;
            if self.tilemap.tiles[linearpos] != SokobanTile::Goal {
                return false;
            }
        }
        return true;
    }
}

struct SokobanView {
    level_id: u32,
    game: SokobanGame,
    callback: Callback,
}

impl SokobanView {
    fn new<F>(level_id: u32, cb: F) -> SokobanView
    where
        F: 'static + Fn(&mut Cursive),
    {
        SokobanView {
            level_id,
            game: load_level(level_id),
            callback: Callback::from_fn(cb),
        }
    }
}

impl cursive::view::View for SokobanView {
    fn draw(&self, printer: &Printer) {
        for (i, t) in self.game.tilemap.tiles.iter().enumerate() {
            printer.print(Vec2::new(i % self.game.tilemap.width, i / self.game.tilemap.width), match t {
                SokobanTile::Empty => " ",
                SokobanTile::Goal => ".",
                SokobanTile::Wall => "#",
            });
        }

        for o in self.game.objects.iter() {
            printer.print(o.pos, match self.game.tilemap.tiles[o.pos.x + o.pos.y * self.game.tilemap.width] {
                SokobanTile::Empty => match o.r#type {
                    SokobanObjectType::Player => "@",
                    SokobanObjectType::Box => "$",
                },
                SokobanTile::Goal => match o.r#type {
                    SokobanObjectType::Player => "+",
                    SokobanObjectType::Box => "*",
                },
                _ => panic!(),
            });
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        return Vec2::new(self.game.tilemap.width, self.game.tilemap.tiles.len() / self.game.tilemap.width);
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('r') => {
                self.game = load_level(self.level_id);

                EventResult::Consumed(None)
            },
            Event::Key(key) => {
                let direction = match key {
                    Key::Left => MovementDirection::Left,
                    Key::Right => MovementDirection::Right,
                    Key::Up => MovementDirection::Up,
                    Key::Down => MovementDirection::Down,
                    _ => return EventResult::Ignored,
                };

                let mut i = 0;
                while i < self.game.objects.len() {
                    if self.game.objects[i].r#type == SokobanObjectType::Player {
                        move_object(&mut self.game.objects, i, direction, &self.game.tilemap);
                        if self.game.has_won() {
                            return EventResult::Consumed(Some(self.callback.clone()));
                        }
                        break;
                    }
                    i += 1;
                }

                EventResult::Consumed(None)
            },
            _ => EventResult::Ignored,
        }
    }
}

fn move_object(objects: &mut Vec<SokobanObject>, which: usize, direction: MovementDirection,
    tilemap: &SokobanTilemap) -> bool {

    let newpos = match direction {
        MovementDirection::Left => objects[which].pos - Vec2::new(1, 0),
        MovementDirection::Right => objects[which].pos + Vec2::new(1, 0),
        MovementDirection::Up => objects[which].pos - Vec2::new(0, 1),
        MovementDirection::Down => objects[which].pos + Vec2::new(0, 1),
    };

    if tilemap.tiles[newpos.x + newpos.y * tilemap.width].issolid() {
        return false
    }

    let mut i = 0;
    while i < objects.len() {
        if objects[i].pos == newpos {
            match objects[which].r#type {
                SokobanObjectType::Box => return false,
                SokobanObjectType::Player => {
                    if move_object(objects, i, direction, tilemap) {
                        break;
                    } else {
                        return false;
                    }
                },
            }
        }
        i += 1;
    }

    objects[which].pos = newpos;
    return true;
}

fn level_exists(id: u32) -> bool {
    let s = &format!("levels/{}.txt", id);
    return Path::new(s).exists();
}

fn load_level(id: u32) -> SokobanGame {
    let s = &format!("levels/{}.txt", id);
    let path = Path::new(s);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut width = 0;
    let mut stuff: Vec<Vec<SokobanTileRaw>> = Vec::new();
    for line in reader.lines() {
        stuff.push(line.unwrap().chars().map(
            |c| match c {
                '#' => SokobanTileRaw::Wall,
                '@' => SokobanTileRaw::Player,
                '+' => SokobanTileRaw::GoalPlayer,
                '$' => SokobanTileRaw::Box,
                '*' => SokobanTileRaw::GoalBox,
                '.' => SokobanTileRaw::Goal,
                ' ' => SokobanTileRaw::Empty,
                _ => panic!(),
            }
        ).collect());
        width = width.max(stuff.last().unwrap().len())
    }

    while stuff.last().unwrap().len() == 0 {
        stuff.pop();
    }

    let mut data: Vec<SokobanTileRaw> = Vec::new();
    for mut thing in stuff {
        let pad = width - thing.len();
        data.append(&mut thing);
        for _ in 0..pad {
            data.push(SokobanTileRaw::Empty);
        }
    }

    SokobanGame::new(width, data)
}

fn main() {
    let gameview = SokobanView::new(1, |s| s.add_layer(
        Dialog::text("You win!").button("Quit", |s| s.quit())
    ));

    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());

    siv.add_layer(
        Dialog::around(gameview)
            .title("Sokoban")
    );

    siv.run();
}
