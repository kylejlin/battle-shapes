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

    pub fn update(&mut self, dt: f64) {

    }
}

pub fn get_team_color(team: &Team) -> [f32; 4] {
    match team {
        &Team::Blue => [0.0, 0.0, 1.0, 1.0],
        &Team::Red => [1.0, 0.0, 0.0, 1.0]
    }
}

pub mod render_properties {
    pub mod swordsman_properties {
        pub const SIZE: f64 = 40.0;
    }
}
