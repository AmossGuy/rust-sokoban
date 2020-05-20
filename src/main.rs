use cursive::Cursive;
use cursive::Printer;
use cursive::XY;
use cursive::event::{Event, EventResult, Key};
use cursive::vec::Vec2;
use cursive::views::Dialog;

#[derive(Clone)]
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

struct SokobanTilemap {
    width: usize,
    tiles: Vec<SokobanTile>,
}

#[derive(PartialEq)]
enum SokobanObjectType {
    Player,
    Box,
}

struct SokobanObject {
    r#type: SokobanObjectType,
    pos: Vec2,
}

impl SokobanObject {
    fn r#move(&mut self, delta: XY<isize>, tilemap: &SokobanTilemap) {
    }
}

struct SokobanGame {
    tilemap: SokobanTilemap,
    objects: Vec<SokobanObject>,
}

impl SokobanGame {
    fn new(width: usize, levelstring: &str) -> SokobanGame {
        assert!(levelstring.chars().count() % width == 0);

        let mut game = SokobanGame {
            tilemap: SokobanTilemap {
                width,
                tiles: vec![SokobanTile::Empty; levelstring.chars().count()],
            },
            objects: Vec::new(),
        };

        for (i, c) in levelstring.chars().enumerate() {
            game.tilemap.tiles[i] = match c {
                ' ' | '$' | '@' => SokobanTile::Empty,
                '.' | '*' | '+' => SokobanTile::Goal,
                '#' => SokobanTile::Wall,
                _ => panic!(),
            };

            match c {
                '@' | '+' => game.objects.push(SokobanObject {
                    r#type: SokobanObjectType::Player,
                    pos: Vec2::new(i % game.tilemap.width, i / game.tilemap.width),
                }),
                '$' | '*' => game.objects.push(SokobanObject {
                    r#type: SokobanObjectType::Box,
                    pos: Vec2::new(i % game.tilemap.width, i / game.tilemap.width),
                }),
                _ => (),
            };
        }

        game
    }
}

struct SokobanView {
    game: SokobanGame,
}

impl SokobanView {
    fn new(game: SokobanGame) -> SokobanView {
        SokobanView {
            game,
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
            Event::Key(key) => {
                let delta = match key {
                    Key::Left => XY::<isize>::new(-1, 0),
                    Key::Right => XY::<isize>::new(1, 0),
                    Key::Up => XY::<isize>::new(0, -1),
                    Key::Down => XY::<isize>::new(0, 1),
                    _ => return EventResult::Ignored,
                };

                for object in &mut self.game.objects {
                    if object.r#type == SokobanObjectType::Player {
                        object.r#move(delta, &self.game.tilemap);
                    }
                }

                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }
}

fn main() {
    let levelstring = concat!("    #####          ",
                              "    #   #          ",
                              "    #$  #          ",
                              "  ###  $##         ",
                              "  #  $ $ #         ",
                              "### # ## #   ######",
                              "#   # ## #####  ..#",
                              "# $  $          ..#",
                              "##### ### #@##  ..#",
                              "    #     #########",
                              "    #######        ");

    let game = SokobanGame::new(19, levelstring);
    let gameview = SokobanView::new(game);

    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        Dialog::around(gameview)
            .title("Sokoban")
    );

    siv.run();
}
