use super::victor::Victor;

pub struct Troop {
    pub team: Team,
    pub troop_type: TroopType,
    pub health: u16,
    pub x: f64,
    pub y: f64
}

pub enum Team {
    Blue,
    Red
}

pub enum TroopType {
    Swordsman
}

impl Troop {
    pub fn new(team: Team, troop_type: TroopType) -> Troop {
        Troop {
            team,
            troop_type,
            health: 100,
            x: 480.0,
            y: 360.0
        }
    }

    pub fn update(&mut self, dt: f64) -> Victor {
        match self.troop_type {
            TroopType::Swordsman => {
                let step = match self.team {
                    Team::Blue => 20.0,
                    Team::Red => -20.0
                };

                self.x += dt * step;
            }
        }

        match self.team {
            Team::Blue => {
                if self.x > 960.0 {
                    return Victor::Blue;
                }
            },
            Team::Red => {
                if self.x < 0.0 {
                    return Victor::Red;
                }
            }
        }

        Victor::None
    }

    pub fn get_size(&self) -> f64 {
        Troop::get_size_of_troop_type(&self.troop_type)
    }

    fn get_size_of_troop_type(troop_type: &TroopType) -> f64 {
        use self::troop_properties::{
            swordsman_properties
        };

        match troop_type {
            &TroopType::Swordsman => swordsman_properties::SIZE
        }
    }
}

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
    pub mod swordsman_properties {
        pub const SIZE: f64 = 40.0;
    }
}
