pub struct Player {
    health: u32,
    attack: u32,
    defense: u32,
}

impl Player {
    pub fn new(health: u32, attack: u32, defense: u32) -> Self {
        Self {
            health,
            attack,
            defense,
        }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            health: 100,
            attack: 5,
            defense: 5,
        }
    }
}
