use super::troops::{
    Troop,
    Team,
    TroopType
};
use super::victor::Victor;

pub struct BattleField {
    pub troops: Vec<Troop>,
    pub victor: Victor
}

impl BattleField {
    pub fn new() -> BattleField {
        BattleField {
            troops: Vec::new(),
            victor: Victor::None
        }
    }

    pub fn update_troop(troop: &mut Troop, dt: f64) -> Victor {
        match troop.troop_type {
            TroopType::Swordsman => {
                let step = match troop.team {
                    Team::Blue => 20.0,
                    Team::Red => -20.0
                };

                troop.x += dt * step;
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

    pub fn add_troop(&mut self, troop: Troop) {
        self.troops.push(troop);
    }

    pub fn update(&mut self, dt: f64) {
        match self.victor {
            Victor::None => {},
            _ => {
                return;
            }
        };

        for troop in &mut self.troops {
            let new_victor = Self::update_troop(troop, dt);

            if let Victor::None = new_victor {
                continue;
            }

            self.victor = new_victor;
            return;
        }
    }
}
