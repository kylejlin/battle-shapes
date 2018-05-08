use super::troops::{
    Troop,
    PendingTroopDeployment,
    Team,
    TroopType,
    troop_properties
};
use super::victor::Victor;

pub struct BattleField {
    pub troops: Vec<Troop>,
    pub victor: Victor,
    id_counter: u32
}

impl BattleField {
    pub fn new() -> BattleField {
        BattleField {
            troops: Vec::new(),
            victor: Victor::None,
            id_counter: 0
        }
    }

    pub fn update_troop(original_troops: &Vec<Troop>, troop: &mut Troop, dt: f64) -> Victor {
        match troop.troop_type {
            TroopType::Swordsman => {
                let step = match troop.team {
                    Team::Blue => 20.0,
                    Team::Red => -20.0
                };

                troop.x += dt * step;

                let enemy_team = troop.team.enemy();
                let mut engaged_troop: Option<&Troop> = None;

                for other_troop in original_troops {
                    if (
                        enemy_team == other_troop.team
                        && Self::are_troops_touching(troop, other_troop)
                    ) {
                        engaged_troop = Some(other_troop);
                        break;
                    }
                }

                if let Some(engaged_troop) = engaged_troop {
                    troop.y += dt * step; // TODO
                }
            }
        }

        match troop.team {
            Team::Blue => {
                if troop.x > 960.0 {
                    return Victor::Blue;
                }
            },
            Team::Red => {
                if troop.x < 0.0 {
                    return Victor::Red;
                }
            }
        }

        Victor::None
    }

    pub fn are_troops_touching(a: &Troop, b: &Troop) -> bool {
        let a_size = troop_properties::get_size_of_troop_type(&a.troop_type);
        let b_size = troop_properties::get_size_of_troop_type(&b.troop_type);
        let max_gap = (a_size + b_size) / 2.0;

        (a.x - b.x).abs() < max_gap
            && (a.y - b.y).abs() < max_gap
    }

    pub fn add_troop(&mut self, troop: PendingTroopDeployment) {
        let id = self.id_counter;
        self.id_counter = id + 1;

        self.troops.push(
            Troop {
                id,
                team: troop.team,
                troop_type: troop.troop_type,
                health: troop.health,
                x: troop.x,
                y: troop.y
            }
        );
    }

    pub fn update(&mut self, dt: f64) {
        match self.victor {
            Victor::None => {},
            _ => {
                return;
            }
        };

        let original_troops = self.troops.clone();

        for troop in &mut self.troops {
            let new_victor = Self::update_troop(&original_troops, troop, dt);

            if let Victor::None = new_victor {
                continue;
            }

            self.victor = new_victor;
            return;
        }
    }
}
