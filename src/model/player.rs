use std::f32::consts::PI;

use super::{damage::Damageable, weapons::Weapon};

pub struct Player {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub weapon: Weapon,
    pub stun: f32,
}

impl Player {
    pub fn new(
        health: f32,
        attack: f32,
        defense: f32,
        speed: f32,
        weapon: Weapon,
        stun: f32,
    ) -> Self {
        Self {
            health,
            attack,
            defense,
            speed,
            weapon,
            stun,
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
                cooldown_counter: 0.,
                knockback: 0.,
                stun: 0.,
                animation: 0.,
                kind: super::weapons::WeaponKind::Melee {
                    range: 100.,
                    angle: PI / 3.,
                },
            },
            stun: 0.,
        }
    }
}

impl Damageable for Player {
    fn take_damage(
        &mut self,
        damage: super::damage::Damage,
        effects: Vec<super::damage::Effects>,
    ) -> crate::errors::Nresult {
        if self.stun <= 0. {
            self.take_damage_raw(damage.evaluate()?)?;
            self.stun = damage.stun;
            Ok(())
        } else {
            Ok(())
        }
    }
    fn take_damage_raw(&mut self, damage: f32) -> crate::errors::Nresult {
        self.health -= damage;
        Ok(())
    }
}
