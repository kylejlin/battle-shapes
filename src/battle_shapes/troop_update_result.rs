use super::victor::Victor;
use super::troops::PendingTroopDeployment;

pub struct TroopUpdateResult {
    pub victor: Victor,
    pub changes: Vec<BattleChange>
}

#[derive(Clone)]
pub enum BattleChange {
    TroopChange(TroopChange),
    TroopDeployment(PendingTroopDeployment)
}

#[derive(Clone)]
pub struct TroopChange {
    pub id: u32,
    pub health: f64,
    pub x: f64,
    pub y: f64,
    pub health_bar_counter: f64
}

impl TroopUpdateResult {
    pub fn zero_change(victor: Victor) -> Self {
        Self {
            victor,
            changes: Vec::new()
        }
    }
}
