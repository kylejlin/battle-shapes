use super::victor::Victor;

#[derive(Clone)]
pub struct Troop {
    pub id: u32,
    pub team: Team,
    pub troop_type: TroopType,
    pub health: f64,
    pub x: f64,
    pub y: f64,
    pub health_bar_counter: f64
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
    Swordsman
}

impl Troop {
    pub fn new(id: u32, team: Team, troop_type: TroopType) -> Troop {
        Troop {
            id,
            team,
            troop_type,
            health: 100.0,
            x: 480.0,
            y: 360.0,
            health_bar_counter: 0.0
        }
    }
}

/*impl std::clone::Clone for Troop {
    pub fn clone(&self) -> Troop {
        Troop {
            team: self.team,
            troop_type: self.troop_type,
            health: self.health,
            x: self.x,
            y: self.y
        }
    }

    pub fn clone_from(&mut self, source: &Troop) {

    }
}*/

impl Team {
    pub fn enemy(&self) -> Team {
        match self {
            &Team::Blue => Team::Red,
            &Team::Red => Team::Blue
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
            swordsman_properties
        };

        match troop_type {
            &TroopType::Swordsman => swordsman_properties::SIZE
        }
    }

    pub mod swordsman_properties {
        pub const SIZE: f64 = 40.0;
    }
}
