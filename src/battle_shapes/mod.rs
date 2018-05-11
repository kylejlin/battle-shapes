extern crate piston_window;
extern crate rand;

mod troops;
mod colors;
mod victor;
mod battle_field;
mod troop_update_result;

use std::env;

use piston_window::{
    PistonWindow,

    Event,

    UpdateArgs,
    Button,
    Key,

    rectangle,
    clear
};
use self::rand::Rng;


use self::troops::{
    PendingTroopDeployment,
    Troop,
    Team,
    TroopType
};
use self::victor::Victor;
use self::battle_field::BattleField;
use self::colors::GRASS;

fn is_sandbox_mode_on() -> bool {
    let args: Vec<String> = env::args().collect();

    args.contains(&String::from("sandbox"))
}

fn is_big_money_mode_on() -> bool {
    let args: Vec<String> = env::args().collect();

    args.contains(&String::from("big-money"))
}

fn rand_int(min_incl: f64, max_excl: f64) -> f64 {
    rand::thread_rng().gen_range(min_incl, max_excl)
}

fn rand_position() -> (f64, f64) {
    (
        rand_int(660.0, 960.0),
        rand_int(0.0, 720.0)
    )
}

pub struct App {
    pub battle_field: BattleField,
    pub cursor: [f64; 2],
    pub blue_coins: f64,
    pub red_coins: f64,
    pub coins_per_second: f64,
    pub max_coins: f64,
    pub border: f64,
    pub is_sandbox_mode_on: bool,
    pub is_big_money_mode_on: bool
}

impl App {
    pub fn new() -> App {
        App {
            battle_field: BattleField::new(),
            cursor: [480.0, 360.0],
            blue_coins: 0.0,
            red_coins: 0.0,
            coins_per_second: 10.0,
            max_coins: 250.0,
            border: 300.0,
            is_sandbox_mode_on: is_sandbox_mode_on(),
            is_big_money_mode_on: is_big_money_mode_on()
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        self.render_background(window, event);
        self.render_borders(window, event);
        self.battle_field.render_troops(window, event);
        self.render_coins(window, event);
    }

    pub fn render_coins(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g| {
            rectangle(
                [1.0, 1.0, 1.0, 0.5],
                [
                    10.0,
                    10.0,
                    100.0,
                    10.0
                ],
                c.transform,
                g
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    10.0,
                    10.0,
                    (self.blue_coins / self.max_coins) * 100.0,
                    10.0
                ],
                c.transform,
                g
            );

            rectangle(
                [1.0, 1.0, 1.0, 0.5],
                [
                    850.0,
                    10.0,
                    100.0,
                    10.0
                ],
                c.transform,
                g
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    850.0,
                    10.0,
                    (self.red_coins / self.max_coins) * 100.0,
                    10.0
                ],
                c.transform,
                g
            );
        });
    }

    pub fn render_borders(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g| {
            rectangle(
                [1.0, 1.0, 1.0, 0.5],
                [
                    self.border,
                    0.0,
                    1.0,
                    720.0
                ],
                c.transform,
                g
            );
            rectangle(
                [1.0, 1.0, 1.0, 0.5],
                [
                    960.0 - self.border,
                    0.0,
                    1.0,
                    720.0
                ],
                c.transform,
                g
            );
        });
    }

    pub fn render_background(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |_c, g| {
            clear(GRASS, g);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.battle_field.update(args.dt);

        let money_multiplier = if self.is_big_money_mode_on {
            5.0
        } else {
            1.0
        };

        self.blue_coins += money_multiplier * self.coins_per_second * args.dt;
        self.red_coins += money_multiplier * self.coins_per_second * args.dt;

        if self.blue_coins > self.max_coins {
            self.blue_coins = self.max_coins;
        }

        if self.red_coins > self.max_coins {
            self.red_coins = self.max_coins;
        }



        if !self.is_sandbox_mode_on {
            self.update_computer(args.dt);
        }
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
                    self.add_blue_troop_if_legal(TroopType::Swordsman);
                },
                Key::D2 => {
                    self.add_blue_troop_if_legal(TroopType::Archer);
                },
                Key::D3 => {
                    self.add_blue_troop_if_legal(TroopType::Rider);
                },
                Key::D4 => {
                    self.add_blue_troop_if_legal(TroopType::Giant);
                },
                Key::D5 => {
                    self.add_blue_troop_if_legal(TroopType::Daggerman);
                },
                Key::Q => {
                    self.add_blue_troop_if_legal(TroopType::Shieldsman);
                },
                Key::A => {
                    self.add_blue_troop_if_legal(TroopType::Wall);
                },
                Key::D0 => {
                    self.add_red_troop_if_legal(TroopType::Swordsman);
                },
                Key::D9 => {
                    self.add_red_troop_if_legal(TroopType::Archer);
                },
                Key::D8 => {
                    self.add_red_troop_if_legal(TroopType::Rider);
                },
                Key::D7 => {
                    self.add_red_troop_if_legal(TroopType::Giant);
                },
                Key::D6 => {
                    self.add_red_troop_if_legal(TroopType::Daggerman);
                },
                Key::P => {
                    self.add_red_troop_if_legal(TroopType::Shieldsman);
                },
                Key::O => {
                    self.add_red_troop_if_legal(TroopType::Wall);
                },
                _ => {}
            }
        }
    }

    pub fn handle_mouse_cursor_move(&mut self, coordinates: [f64; 2]) {
        self.cursor = coordinates;
    }

    // "legal" = affordable and onsides

    pub fn add_blue_troop_if_legal(&mut self, troop_type: TroopType) {
        let cost = troop_type.get_cost();

        if self.blue_coins >= cost
            &&self.cursor[0] <= self.border
        {
            self.blue_coins -= cost;

            self.battle_field.add_troop(
                PendingTroopDeployment{
                    team: Team::Blue,
                    troop_type,
                    x: self.cursor[0],
                    y: self.cursor[1]
                }
            );
        }
    }

    // "legal" = affordable and onsides and is_sandbox_mode_on==true

    pub fn add_red_troop_if_legal(&mut self, troop_type: TroopType) {
        let cost = troop_type.get_cost();

        if self.red_coins >= cost
            &&self.cursor[0] >= (960.0 - self.border)
            &&self.is_sandbox_mode_on
        {
            self.red_coins -= cost;

            self.battle_field.add_troop(
                PendingTroopDeployment{
                    team: Team::Red,
                    troop_type,
                    x: self.cursor[0],
                    y: self.cursor[1]
                }
            );
        }

    }

    fn force_add_red_troop(&mut self, troop_type: TroopType, x: f64, y: f64) {
        let cost = troop_type.get_cost();
        self.red_coins -= cost;

        self.battle_field.add_troop(
            PendingTroopDeployment{
                team: Team::Red,
                troop_type,
                x,
                y
            }
        );
    }

    pub fn update_computer(&mut self, dt: f64) {
        let position = rand_position();

        let troops = self.battle_field.troops.clone();
        let mut threat: Option<&Troop> = None;

        for troop in &troops {
            if let Team::Red = troop.team {
                continue;
            }

            if troop.x > 660.0 {
                if let Some(current_threat) = threat {
                    if troop.x > current_threat.x {
                        threat = Some(troop);
                    }
                } else {
                    threat = Some(troop);
                }
            }
        }

        if let Some(threat) = threat {
            if self.red_coins > TroopType::Wall.get_cost() {
                self.force_add_red_troop(
                    TroopType::Wall,
                    threat.x,
                    threat.y
                );
            }
        } else {
            if self.red_coins > 200.0 {
                self.force_add_red_troop(
                    TroopType::Swordsman,
                    position.0,
                    position.1
                );
            }
        }
    }
}
