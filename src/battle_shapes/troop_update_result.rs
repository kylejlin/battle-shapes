use super::victor::Victor;

pub struct TroopUpdateResult {
    pub victor: Victor,
    pub changes: Vec<TroopChange>
}

pub struct TroopChange {
    pub id: u32,
    pub health: f64,
    pub x: f64,
    pub y: f64
}

impl TroopUpdateResult {
    pub fn zero_change(victor: Victor) -> Self {
        Self {
            victor,
            changes: Vec::new()
        }
    }
}
