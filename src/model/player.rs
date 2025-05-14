pub struct Player {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(health: f32, attack: f32, defense: f32, speed: f32) -> Self {
        Self {
            health,
            attack,
            defense,
            speed,
        }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            health: 100.,
            attack: 5.,
            defense: 5.,
            speed: 250.,
        }
    }
}
