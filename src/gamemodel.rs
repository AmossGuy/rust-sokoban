use cursive::vec::Vec2;
use std::cmp::max;
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

pub struct Object {
    pub pos: Vec2,
    pub kind: ObjectKind,
}

pub struct GameModel {
    pub tilemap: Vec<Vec<Tile>>,
    pub objects: Vec<Object>,
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
                row.push(match c {
                    ' ' | '@' | '$' => Tile::Floor,
                    '.' | '+' | '*' => Tile::Goal,
                    '#' => Tile::Wall,
                    _ => panic!(),
                });
            }
            tilemap.push(row);
        }

        while tilemap.last().unwrap().len() == 0 {
            tilemap.pop();
        }

        GameModel {
            tilemap,
            objects,
        }
    }

    pub fn get_level_extents(&self) -> Vec2 {
        let mut width = 0;
        for row in self.tilemap.iter() {
            width = max(width, row.len());
        }

        Vec2::new(width, self.tilemap.len())
    }
}
