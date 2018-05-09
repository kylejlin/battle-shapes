#[derive(Clone)]
pub struct Troop {
    pub id: u32,
    pub team: Team,
    pub troop_type: TroopType,
    pub health: f64,
    pub x: f64,
    pub y: f64,
    pub health_bar_counter: f64,
    pub attack_cooldown: f64
}

pub struct PendingTroopDeployment {
    pub team: Team,
    pub troop_type: TroopType,
    pub health: f64,
    pub x: f64,
    pub y: f64
}

#[derive(Clone, PartialEq)]
pub enum Team {
    Blue,
    Red
}

#[derive(Clone)]
pub enum TroopType {
    Swordsman,
    Wall
}

impl Team {
    pub fn enemy(&self) -> Team {
        match self {
            &Team::Blue => Team::Red,
            &Team::Red => Team::Blue
        }
    }
}

impl TroopType {
    pub fn is_attackable(&self) -> bool {
        match self {
            &TroopType::Swordsman => true,
            &TroopType::Wall => true
        }
    }

    pub fn is_movable(&self) -> bool {
        match self {
            &TroopType::Swordsman => true,
            &TroopType::Wall => false
        }
    }
}

pub fn get_team_color(team: &Team) -> [f32; 4] {
    match team {
        &Team::Blue => [0.0, 0.0, 1.0, 1.0],
        &Team::Red => [1.0, 0.0, 0.0, 1.0]
    }
}

pub mod troop_properties {
    use super::TroopType;

    pub fn get_size_of_troop_type(troop_type: &TroopType) -> f64 {
        use self::{
            swordsman_properties,
            wall_properties
        };

        match troop_type {
            &TroopType::Swordsman => swordsman_properties::SIZE,
            &TroopType::Wall => wall_properties::SIZE
        }
    }

    pub fn get_damage_of_troop_type(troop_type: &TroopType) -> f64 {
        use self::{
            swordsman_properties
        };

        match troop_type {
            &TroopType::Swordsman => swordsman_properties::DAMAGE,
            &TroopType::Wall => 0.0
        }
    }

    pub fn get_cooldown_of_troop_type(troop_type: &TroopType) -> f64 {
        use self::{
            swordsman_properties
        };

        match troop_type {
            &TroopType::Swordsman => swordsman_properties::COOLDOWN,
            &TroopType::Wall => 0.0
        }
    }

    pub mod swordsman_properties {
        pub const SIZE: f64 = 40.0;
        pub const DAMAGE: f64 = 30.0;
        pub const COOLDOWN: f64 = 0.8;
    }

    pub mod wall_properties {
        pub const SIZE: f64 = 40.0;
    }
}
