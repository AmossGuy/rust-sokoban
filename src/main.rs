use cursive::{Cursive, CursiveExt};
use cursive::event::Key;
use cursive::view::Nameable;
use cursive::views::Dialog;

mod gamemodel;
mod gameview;
use gameview::GameView;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv.add_layer(Dialog::around(
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

    siv.run();
}
