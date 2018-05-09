extern crate piston_window;

mod troops;
mod colors;
mod victor;
mod battle_field;
mod troop_update_result;

use piston_window::{
    PistonWindow,

    Event,

    UpdateArgs,
    Button,
    Key
};
use self::troops::{
    PendingTroopDeployment,
    Team,
    TroopType
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
        self.battle_field.render(window, event);
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
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::D2 => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment {
                            team: Team::Blue,
                            troop_type: TroopType::Wall,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::D3 => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment{
                            team: Team::Blue,
                            troop_type: TroopType::Giant,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::D4 => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment{
                            team: Team::Blue,
                            troop_type: TroopType::Archer,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::P => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment {
                            team: Team::Red,
                            troop_type: TroopType::Swordsman,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::O => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment {
                            team: Team::Red,
                            troop_type: TroopType::Wall,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::I => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment{
                            team: Team::Red,
                            troop_type: TroopType::Giant,
                            x: self.cursor[0],
                            y: self.cursor[1]
                        }
                    );
                },
                Key::U => {
                    self.battle_field.add_troop(
                        PendingTroopDeployment{
                            team: Team::Red,
                            troop_type: TroopType::Archer,
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
}
