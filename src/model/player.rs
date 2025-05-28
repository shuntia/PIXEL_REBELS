use std::f32::consts::PI;

use super::weapons::Weapon;

pub struct Player {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub weapon: Weapon,
}

impl Player {
    pub fn new(health: f32, attack: f32, defense: f32, speed: f32, weapon: Weapon) -> Self {
        Self {
            health,
            attack,
            defense,
            speed,
            weapon,
        }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            health: 100.,
            attack: 5.,
            defense: 5.,
            speed: 100.,
            weapon: Weapon {
                power: 10.,
                crit_chance: 0.01,
                cooldown: 0.5,
                knockback: 0.,
                stun: 0.,
                animation: 0.,
                kind: super::weapons::WeaponKind::Melee {
                    range: 100.,
                    angle: PI / 3.,
                },
            },
        }
    }
}
