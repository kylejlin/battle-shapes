extern crate piston_window;

mod battle_shapes;

use piston_window::{
    PistonWindow,
    WindowSettings,

    RenderEvent,
    UpdateEvent,
    PressEvent,

    clear
};
use battle_shapes::App;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Battle Shapes",
        [960, 720]
    )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new();

    while let Some(e) = window.next() {
        if let Some(render_args) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(update_args) = e.update_args() {
            app.update(&update_args);
        }

        if let Some(button) = e.press_args() {
            app.handle_button_press(&button);
        }
    }
}
