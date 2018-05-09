extern crate piston_window;

use piston_window::{
    PistonWindow,
    Event,

    clear,
    rectangle
};

use super::troops::{
    Troop,
    PendingTroopDeployment,
    Team,
    TroopType
};
use super::victor::Victor;
use super::troop_update_result::{
    TroopUpdateResult,
    TroopChange
};
use super::colors::{
    GRASS,
    IRON,
    WOOD,
    STONE,
    HEALTH_BAR,
    HEALTH_BAR_SECONDARY
};

pub struct BattleField {
    pub troops: Vec<Troop>,
    pub victor: Victor,
    id_counter: u32
}

const HEALTH_BAR_FADE_RATE: f64 = 35.0;

impl BattleField {
    pub fn new() -> BattleField {
        BattleField {
            troops: Vec::new(),
            victor: Victor::None,
            id_counter: 0
        }
    }

    pub fn update_troop(original_troops: &Vec<Troop>, troop: &mut Troop, dt: f64) -> TroopUpdateResult {
        let mut result = TroopUpdateResult::zero_change(Victor::None);

        if troop.health_bar_counter > dt * HEALTH_BAR_FADE_RATE {
            troop.health_bar_counter -= dt * HEALTH_BAR_FADE_RATE;
        } else {
            troop.health_bar_counter = 0.0;
        }

        if troop.attack_cooldown > dt {
            troop.attack_cooldown -= dt;
        } else {
            troop.attack_cooldown = 0.0;
        }

        match troop.troop_type {
            TroopType::Swordsman | TroopType::Giant => {
                let step = match troop.team {
                    Team::Blue => 20.0,
                    Team::Red => -20.0
                };

                troop.x += dt * step;

                let enemy_team = troop.team.enemy();
                let mut engaged_troop: Option<&Troop> = None;

                for other_troop in original_troops {
                    if enemy_team == other_troop.team
                        && Self::are_troops_touching(troop, other_troop)
                        && other_troop.troop_type.is_attackable()
                    {
                        engaged_troop = Some(other_troop);
                        break;
                    }
                }

                if let Some(engaged_troop) = engaged_troop {
                    let is_movable = engaged_troop.troop_type.is_movable();

                    let vert_step = if troop.y > engaged_troop.y {
                        -step.abs()
                    } else {
                        step.abs()
                    };

                    let damage = if troop.attack_cooldown == 0.0 {
                        troop.attack_cooldown = troop.troop_type.get_cooldown();
                        troop.troop_type.get_damage()
                    } else {
                        0.0
                    };

                    if !is_movable {
                        troop.x -= dt * 3.0 * step;
                        troop.y -= dt * vert_step;
                    }

                    result.changes.push(
                        TroopChange {
                            id: engaged_troop.id,
                            x: dt * step * 3.0 * (is_movable as u8 as f64),
                            y: dt * vert_step * (is_movable as u8 as f64),
                            health: -damage,
                            health_bar_counter: damage
                        }
                    );
                }
            },
            TroopType::Wall => {}
        }

        match troop.team {
            Team::Blue => {
                if troop.x > 960.0 {
                    return TroopUpdateResult::zero_change(Victor::Blue);
                }
            },
            Team::Red => {
                if troop.x < 0.0 {
                    return TroopUpdateResult::zero_change(Victor::Red);
                }
            }
        }

        result
    }

    pub fn are_troops_touching(a: &Troop, b: &Troop) -> bool {
        let a_size = a.troop_type.get_size();
        let b_size = b.troop_type.get_size();
        let max_gap = (a_size + b_size) / 2.0;

        (a.x - b.x).abs() < max_gap
            && (a.y - b.y).abs() < max_gap
    }

    fn render_troop(troop: &Troop, window: &mut PistonWindow, event: &Event) {
        let troop_size = troop.troop_type.get_size();
        let team_color = troop.team.get_color();

        match troop.troop_type {
            TroopType::Swordsman | TroopType::Giant => {
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

                    if troop.health_bar_counter > 0.0 {
                        let max_health = troop.troop_type.get_max_health();
                        let health_bar_width = troop_size * troop.health / max_health;
                        let health_bar_secondary_width = troop_size * troop.health_bar_counter / max_health;

                        rectangle(
                            HEALTH_BAR,
                            [
                                troop.x - (troop_size * 0.5),
                                troop.y - (troop_size * 0.8),
                                health_bar_width,
                                troop_size * 0.1
                            ],
                            c.transform,
                            g
                        );
                        rectangle(
                            HEALTH_BAR_SECONDARY,
                            [
                                troop.x - (troop_size * 0.5) + (health_bar_width),
                                troop.y - (troop_size * 0.8),
                                health_bar_secondary_width,
                                troop_size * 0.1
                            ],
                            c.transform,
                            g
                        );
                    }
                });
            },
            TroopType::Wall => {
                window.draw_2d(event, |c, g| {
                    rectangle(
                        STONE,
                        [
                            troop.x - (troop_size * 0.5),
                            troop.y - (troop_size * 0.5),
                            troop_size,
                            troop_size
                        ],
                        c.transform,
                        g
                    );
                    rectangle(
                        team_color,
                        [
                            troop.x - (troop_size * 0.25),
                            troop.y - (troop_size * 0.25),
                            troop_size * 0.5,
                            troop_size * 0.5
                        ],
                        c.transform,
                        g
                    );

                    if troop.health_bar_counter > 0.0 {
                        let max_health = troop.troop_type.get_max_health();
                        let health_bar_width = troop_size * troop.health / max_health;
                        let health_bar_secondary_width = troop_size * troop.health_bar_counter / max_health;

                        rectangle(
                            HEALTH_BAR,
                            [
                                troop.x - (troop_size * 0.5),
                                troop.y - (troop_size * 0.8),
                                health_bar_width,
                                troop_size * 0.1
                            ],
                            c.transform,
                            g
                        );
                        rectangle(
                            HEALTH_BAR_SECONDARY,
                            [
                                troop.x - (troop_size * 0.5) + health_bar_width,
                                troop.y - (troop_size * 0.8),
                                health_bar_secondary_width,
                                troop_size * 0.1
                            ],
                            c.transform,
                            g
                        );
                    }
                });
            }
        }
    }

    pub fn add_troop(&mut self, troop: PendingTroopDeployment) {
        let id = self.id_counter;
        self.id_counter = id + 1;

        self.troops.push(
            Troop {
                id,
                team: troop.team,
                health: troop.troop_type.get_max_health(),
                troop_type: troop.troop_type,
                x: troop.x,
                y: troop.y,
                health_bar_counter: 0.0,
                attack_cooldown: 0.0
            }
        );
    }

    pub fn render(&self, window: &mut PistonWindow, event: &Event) {
        match self.victor {
            Victor::None => {
                window.draw_2d(event, |_c, g| {
                    clear(GRASS, g);
                });

                for troop in &self.troops {
                    Self::render_troop(troop, window, event);
                }
            },
            Victor::Blue => {
                window.draw_2d(event, |_c, g| {
                    clear(Team::Blue.get_color(), g);
                });
            },
            Victor::Red => {
                window.draw_2d(event, |_c, g| {
                    clear(Team::Red.get_color(), g);
                });
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.victor {
            Victor::None => {},
            _ => {
                return;
            }
        };

        let original_troops = self.troops.clone();
        let mut changes_list: Vec<Vec<TroopChange>> = Vec::new();

        for troop in &mut self.troops {
            let result = Self::update_troop(&original_troops, troop, dt);
            let new_victor = result.victor;
            let change = result.changes;

            if let Victor::None = new_victor {
                changes_list.push(change);
                continue;
            }

            self.victor = new_victor;
            return;
        }

        for mut changes in changes_list {
            for change in &mut changes {
                for troop in &mut self.troops {
                    if troop.id == change.id {
                        troop.health += change.health;
                        troop.x += change.x;
                        troop.y += change.y;
                        troop.health_bar_counter += change.health_bar_counter;
                        break;
                    }
                }
            }
        }

        self.troops.retain(|ref t| t.health > 0.0);
    }
}
