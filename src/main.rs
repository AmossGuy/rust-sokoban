use cursive::{Cursive, CursiveExt};
use cursive::event::Key;
use cursive::view::Nameable;
use cursive::views::{Dialog, TextView};

mod gamemodel;
mod gameview;
use gameview::GameView;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.quit());

    show_intro(&mut siv);

    siv.run();
}

fn show_intro(s: &mut Cursive) {
    let message = "\
        Sokoban in Rust by AmossGuy\n\
        \n\
        Arrow keys or WASD to move\n\
        R to restart level, U to undo\n\
        Q to quit\n\
    ";

    let dialog = Dialog::new()
        .title("Sokoban")
        .content(TextView::new(message))
        .button("Play", |s| {s.pop_layer(); show_game(s)})
        .button("Quit", |s| s.quit());

    s.add_layer(dialog);
}

fn show_game(s: &mut Cursive) {
    s.add_layer(Dialog::around(
        GameView::new(1, |s| {
            let mut dialog = Dialog::text("You win!");
            if s.call_on_name("gameview", |view: &mut GameView| {
                view.has_another_level()
            }) == Some(true) {
                dialog.add_button("Next", |s| {
                    s.pop_layer();
                    s.call_on_name("gameview", |view: &mut GameView| {
                        view.load_level(view.get_level() + 1);
                    });
                });
            } else {
                dialog.add_button("Quit", |s| s.quit());
            }
            s.add_layer(dialog);
        }).with_name("gameview")
    ).title("Sokoban"));
}
