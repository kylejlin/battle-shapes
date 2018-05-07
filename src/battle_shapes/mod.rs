extern crate piston_window;

mod troops;
mod colors;
mod victor;

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
    render_properties,
    get_team_color
};
use self::colors::{
    GRASS,
    IRON,
    WOOD
};
use self::victor::Victor;

pub struct App {
    pub troops: Vec<Troop>,
    pub cursor: [f64; 2],
    pub victor: Victor
}

impl App {
    pub fn new() -> App {
        App {
            troops: Vec::new(),
            cursor: [0.0, 0.0],
            victor: Victor::None
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        match self.victor {
            Victor::None => {
                window.draw_2d(event, |_c, g| {
                    clear(GRASS, g);
                });

                for troop in &self.troops {
                    self.render_troop(troop, window, event);
                }
            },
            Victor::Blue => {
                window.draw_2d(event, |_c, g| {
                    clear(get_team_color(&Team::Blue), g);
                });
            },
            Victor::Red => {
                window.draw_2d(event, |_c, g| {
                    clear(get_team_color(&Team::Red), g);
                });
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.victor {
            Victor::None => {
                for troop in &mut self.troops {
                    troop.update(args.dt);
                }
            },
            _ => {}
        }
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
                use self::troops::render_properties::swordsman_properties::SIZE;

                let team_color = get_team_color(&troop.team);

                window.draw_2d(event, |c, g| {
                    rectangle(
                        team_color,
                        [
                            troop.x - (SIZE / 2.0),
                            troop.y - (SIZE / 2.0),
                            SIZE,
                            SIZE
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        IRON,
                        [
                            troop.x - (SIZE * 0.05),
                            troop.y - (SIZE * 0.35),
                            SIZE * 0.1,
                            SIZE * 0.7
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        IRON,
                        [
                            troop.x - (SIZE * 0.15),
                            troop.y + (SIZE * 0.1),
                            SIZE * 0.3,
                            SIZE * 0.1
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        WOOD,
                        [
                            troop.x - (SIZE * 0.05),
                            troop.y + (SIZE * 0.2),
                            SIZE * 0.1,
                            SIZE * 0.15
                        ],
                        c.transform,
                        g
                    );
                });
            }
        }
    }
}
