extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
    PressEvent
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [960, 720]
    )
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {

        }
    }
}
