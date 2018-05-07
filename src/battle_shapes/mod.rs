extern crate piston_window;

use piston_window::{
    PistonWindow,

    Event,

    RenderArgs,
    UpdateArgs,
    Button,

    clear
};

pub struct App {

}

impl App {
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn handle_button_press(&mut self, args: &Button) {

    }
}
