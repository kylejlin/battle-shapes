extern crate piston_window;

mod troops;
mod colors;
mod victor;
mod battle_field;
mod troop_update_result;

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
    PendingTroopDeployment,
    Team,
    TroopType,
    get_team_color,
    troop_properties
};
use self::colors::{
    GRASS,
    IRON,
    WOOD
};
use self::victor::Victor;
use self::battle_field::BattleField;

pub struct App {
    pub battle_field: BattleField,
    pub cursor: [f64; 2]
}

impl App {
    pub fn new() -> App {
        App {
            battle_field: BattleField::new(),
            cursor: [0.0, 0.0]
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        match self.battle_field.victor {
            Victor::None => {
                window.draw_2d(event, |_c, g| {
                    clear(GRASS, g);
                });

                for troop in &self.battle_field.troops {
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
        self.battle_field.update(args.dt);
    }

    pub fn handle_button_press(&mut self, args: &Button) {
        match self.battle_field.victor {
            Victor::None => {},
            _ => {
                return;
            }
        }

        if let &Button::Keyboard(key) = args {
            match key {
                Key::D1 => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment {
                            team: Team::Blue,
                            troop_type: TroopType::Swordsman,
                            health: 100.0,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    )
                },
                Key::P => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment {
                            team: Team::Red,
                            troop_type: TroopType::Swordsman,
                            health: 100.0,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    )
                }
                _ => {}
            }
        }
    }

    pub fn handle_mouse_cursor_move(&mut self, coordinates: [f64; 2]) {
        self.cursor = coordinates;
    }

    fn render_troop(&self, troop: &Troop, window: &mut PistonWindow, event: &Event) {
        let troop_size = troop_properties::get_size_of_troop_type(&troop.troop_type);

        match troop.troop_type {
            TroopType::Swordsman => {
                let team_color = get_team_color(&troop.team);

                window.draw_2d(event, |c, g| {
                    rectangle(
                        team_color,
                        [
                            troop.x - (troop_size / 2.0),
                            troop.y - (troop_size / 2.0),
                            troop_size,
                            troop_size
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        IRON,
                        [
                            troop.x - (troop_size * 0.05),
                            troop.y - (troop_size * 0.35),
                            troop_size * 0.1,
                            troop_size * 0.7
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        IRON,
                        [
                            troop.x - (troop_size * 0.15),
                            troop.y + (troop_size * 0.1),
                            troop_size * 0.3,
                            troop_size * 0.1
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        WOOD,
                        [
                            troop.x - (troop_size * 0.05),
                            troop.y + (troop_size * 0.2),
                            troop_size * 0.1,
                            troop_size * 0.15
                        ],
                        c.transform,
                        g
                    );
                });
            }
        }
    }
}
