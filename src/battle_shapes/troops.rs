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

#[derive(Clone)]
pub struct PendingTroopDeployment {
    pub team: Team,
    pub troop_type: TroopType,
    pub x: f64,
    pub y: f64
}

#[derive(Clone, PartialEq)]
pub enum Team {
    Blue,
    Red
}

#[derive(Clone, PartialEq)]
pub enum TroopType {
    Swordsman,
    Wall,
    Giant,
    Archer,
    Arrow,
    Daggerman,
    Rider,
    Shieldsman,
    CowardArcher
}

impl Team {
    pub fn enemy(&self) -> Team {
        match self {
            &Team::Blue => Team::Red,
            &Team::Red => Team::Blue
        }
    }

    pub fn get_color(&self) -> [f32; 4] {
        match self {
            &Team::Blue => [0.0, 0.0, 1.0, 1.0],
            &Team::Red => [1.0, 0.0, 0.0, 1.0]
        }
    }
}

impl TroopType {
    pub fn is_attackable(&self) -> bool {
        match self {
            &TroopType::Swordsman => true,
            &TroopType::Wall => true,
            &TroopType::Giant => true,
            &TroopType::Archer => true,
            &TroopType::Arrow => false,
            &TroopType::Daggerman => true,
            &TroopType::Rider => true,
            &TroopType::Shieldsman => true,
            &TroopType::CowardArcher => true
        }
    }

    pub fn is_movable(&self) -> bool {
        match self {
            &TroopType::Swordsman => true,
            &TroopType::Wall => false,
            &TroopType::Giant => true,
            &TroopType::Archer => true,
            &TroopType::Arrow => false,
            &TroopType::Daggerman => true,
            &TroopType::Rider => true,
            &TroopType::Shieldsman => true,
            &TroopType::CowardArcher => true
        }
    }

    pub fn get_max_health(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 100.0,
            &TroopType::Wall => 300.0,
            &TroopType::Giant => 1000.0,
            &TroopType::Archer => 100.0,
            &TroopType::Arrow => 1.0,
            &TroopType::Daggerman => 50.0,
            &TroopType::Rider => 100.0,
            &TroopType::Shieldsman => 250.0,
            &TroopType::CowardArcher => 100.0
        }
    }

    pub fn get_size(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 40.0,
            &TroopType::Wall => 40.0,
            &TroopType::Giant => 80.0,
            &TroopType::Archer => 40.0,
            &TroopType::Arrow => 10.0,
            &TroopType::Daggerman => 40.0,
            &TroopType::Rider => 40.0,
            &TroopType::Shieldsman => 40.0,
            &TroopType::CowardArcher => 30.0
        }
    }

    pub fn get_damage(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 30.0,
            &TroopType::Wall => 0.0,
            &TroopType::Giant => 100.0,
            &TroopType::Archer => 0.0,
            &TroopType::Arrow => 25.0,
            &TroopType::Daggerman => 20.0,
            &TroopType::Rider => 25.0,
            &TroopType::Shieldsman => 0.0,
            &TroopType::CowardArcher => 0.0
        }
    }

    pub fn get_cooldown(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 0.8,
            &TroopType::Wall => 0.0,
            &TroopType::Giant => 3.5,
            &TroopType::Archer => 2.0,
            &TroopType::Arrow => 0.0,
            &TroopType::Daggerman => 0.15,
            &TroopType::Rider => 1.0,
            &TroopType::Shieldsman => 1.0,
            &TroopType::CowardArcher => 2.75
        }
    }

    pub fn get_cost(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 30.0,
            &TroopType::Wall => 10.0,
            &TroopType::Giant => 100.0,
            &TroopType::Archer => 60.0,
            &TroopType::Arrow => 0.0,
            &TroopType::Daggerman => 50.0,
            &TroopType::Rider => 50.0,
            &TroopType::Shieldsman => 40.0,
            &TroopType::CowardArcher => 60.0
        }
    }

    pub fn get_abs_step(&self) -> f64 {
        match self {
            &TroopType::Swordsman => 35.0,
            &TroopType::Wall => 0.0,
            &TroopType::Giant => 12.0,
            &TroopType::Archer => 20.0,
            &TroopType::Arrow => 1000.0,
            &TroopType::Daggerman => 25.0,
            &TroopType::Rider => 80.0,
            &TroopType::Shieldsman => 35.0,
            &TroopType::CowardArcher => 15.0
        }
    }
}
