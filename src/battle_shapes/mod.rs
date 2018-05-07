extern crate piston_window;

mod troops;

use piston_window::{
    PistonWindow,

    Event,

    RenderArgs,
    UpdateArgs,
    Button,

    clear
};
use self::troops::Troop;

const FIELD_COLOR: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

pub struct App {
    pub troops: Vec<Troop>
}

impl App {
    pub fn new() -> App {
        App {
            troops: Vec::new()
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |_c, g| {
            clear(FIELD_COLOR, g);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn handle_button_press(&mut self, args: &Button) {

    }
}
