use cursive::vec::Vec2;
use cursive::XY;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor,
    Goal,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Player,
    Box,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
}

pub struct Object {
    pub pos: Vec2,
    pub kind: ObjectKind,
}

pub struct GameModel {
    pub tilemap: Vec<Vec<Tile>>,
    pub objects: Vec<Object>,
    undo_states: Vec<HashMap<usize, Vec2>>,
}

impl GameModel {
    pub fn new(level_id: usize) -> Self {
        let path_s = &format!("levels/{}.txt", level_id);
        let path = Path::new(path_s);
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let mut tilemap: Vec<Vec<Tile>> = vec![];
        let mut objects: Vec<Object> = vec![];
        for line in reader.lines() {
            let mut row = vec![];
            for c in line.unwrap().chars() {
                let x = row.len();
                let y = tilemap.len();

                row.push(match c {
                    ' ' | '@' | '$' => Tile::Floor,
                    '.' | '+' | '*' => Tile::Goal,
                    '#' => Tile::Wall,
                    _ => panic!(),
                });

                match c {
                    '@' | '+' => objects.push(Object {
                        pos: Vec2::new(x, y),
                        kind: ObjectKind::Player,
                    }),
                    '$' | '*' => objects.push(Object {
                        pos: Vec2::new(x, y),
                        kind: ObjectKind::Box,
                    }),
                    _ => (),
                }
            }
            tilemap.push(row);
        }

        while tilemap.last().unwrap().len() == 0 {
            tilemap.pop();
        }

        GameModel {
            tilemap,
            objects,
            undo_states: Vec::new(),
        }
    }

    pub fn get_level_extents(&self) -> Vec2 {
        let mut width = 0;
        for row in self.tilemap.iter() {
            width = max(width, row.len());
        }

        Vec2::new(width, self.tilemap.len())
    }

    pub fn do_action(&mut self, action: Action) {
        let delta = match action {
            Action::Left => XY::<isize>::new(-1, 0),
            Action::Right => XY::<isize>::new(1, 0),
            Action::Up => XY::<isize>::new(0, -1),
            Action::Down => XY::<isize>::new(0, 1),
        };

        self.undo_states.push(HashMap::new());

        let mut i = 0;
        while i < self.objects.len() {
            if self.objects[i].kind == ObjectKind::Player {
                self.move_object(i, delta);
                break;
            }
            i += 1;
        }
    }

    fn move_object(&mut self, object_index: usize, delta: XY::<isize>) -> bool {
        let state = self.undo_states.last_mut().unwrap();
        state.insert(object_index, self.objects[object_index].pos);

        let newpos = Vec2::new(
            u_plus_i(self.objects[object_index].pos.x, delta.x),
            u_plus_i(self.objects[object_index].pos.y, delta.y),
        );

        if self.tilemap[newpos.y][newpos.x] == Tile::Wall {
            return false;
        }

        let mut i = 0;
        while i < self.objects.len() {
            if self.objects[i].pos == newpos {
                match self.objects[object_index].kind {
                    ObjectKind::Box => return false,
                    ObjectKind::Player => {
                        if self.move_object(i, delta) {
                            break;
                        } else {
                            return false;
                        }
                    },
                }
            }
            i += 1;
        }

        self.objects[object_index].pos = newpos;
        return true;
    }

    pub fn undo(&mut self) {
        let state = match self.undo_states.pop() {
            Some(x) => x,
            None => return,
        };

        for (key, value) in state {
            self.objects[key].pos = value;
        }
    }

    pub fn has_won(&self) -> bool {
        for object in self.objects.iter() {
            if object.kind == ObjectKind::Box {
                if self.tilemap[object.pos.y][object.pos.x] != Tile::Goal {
                    return false;
                }
            }
        }
        return true;
    }
}

fn u_plus_i(u: usize, i: isize) -> usize {
    if i < 0 {
        return u - (-i as usize);
    } else {
        return u + (i as usize);
    }
}
