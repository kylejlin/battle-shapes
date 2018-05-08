use super::troops::{
    Troop,
    PendingTroopDeployment,
    Team,
    TroopType,
    troop_properties
};
use super::victor::Victor;
use super::troop_update_result::{
    TroopUpdateResult,
    TroopChange
};

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

    pub fn apply_change(troop: &mut Troop, change: &TroopChange) {

    }

    pub fn update_troop(original_troops: &Vec<Troop>, troop: &mut Troop, dt: f64) -> TroopUpdateResult {
        let mut result = TroopUpdateResult::zero_change(Victor::None);

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
                    troop.y += dt * step;
                    result.changes.push(
                        TroopChange {
                            id: engaged_troop.id,
                            x: dt * step * 3.0,
                            y: 0.0,
                            health: dt * -20.0
                        }
                    );
                }
            }
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
                        break;
                    }
                }
            }
        }
    }
}
