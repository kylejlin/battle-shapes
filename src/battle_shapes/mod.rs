extern crate piston_window;

mod troops;

use piston_window::{
    PistonWindow,

    Event,

    RenderArgs,
    UpdateArgs,
    Button,
    Key,

    clear,
    rectangle
};
use self::troops::{
    Troop,
    Team,
    TroopType,
    render_properties
};
const FIELD_COLOR: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

pub struct App {
    pub troops: Vec<Troop>,
    pub cursor: [f64; 2]
}

impl App {
    pub fn new() -> App {
        App {
            troops: Vec::new(),
            cursor: [0.0, 0.0]
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |_c, g| {
            clear(FIELD_COLOR, g);
        });

        for troop in &self.troops {
            self.render_troop(troop, window, event);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn handle_button_press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::D1 => {
                    self.troops.push(
                        Troop {
                            team: Team::Blue,
                            troop_type: TroopType::Swordsman,
                            health: 100,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                _ => {}
            }
        }
    }

    pub fn handle_mouse_cursor_move(&mut self, coordinates: [f64; 2]) {
        self.cursor = coordinates;
    }

    fn render_troop(&self, troop: &Troop, window: &mut PistonWindow, event: &Event) {
        match troop.troop_type {
            TroopType::Swordsman => {
                use self::troops::render_properties::swordsman_properties::{
                    COLOR,
                    SIZE
                };

                window.draw_2d(event, |c, g| {
                    rectangle(
                        COLOR,
                        [
                            troop.x - (SIZE / 2.0),
                            troop.y - (SIZE / 2.0),
                            SIZE,
                            SIZE
                        ],
                        c.transform,
                        g
                    );
                });
            }
        }
    }
}
