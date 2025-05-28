use rand::random;

use crate::{
    errors::{Nresult, Result},
    model::weapons::Weapon,
};

const CRIT_MULTIPLIER: f32 = 2.0;

pub enum Effects {}

pub trait Damageable {
    /// Take damage from character `from`
    fn take_damage(&mut self, damage: Damage, effects: Vec<Effects>) -> Nresult;
    /// Ignore all effects/multipliers, and take raw damage
    fn take_damage_raw(&mut self, damage: f32) -> Nresult;
}

pub struct Damage<'a> {
    /// raw attack power value
    raw: f32,
    /// attack buff
    buff: f32,
    /// weapon being used
    weapon: Option<&'a Weapon>,
    /// always crit
    crit_override: bool,
    /// calculated cache
    calculated: Option<f32>,
}

impl<'a> Damage<'a> {
    /// evaluate damage based on parameters
    pub fn evaluate(&self) -> Result<f32> {
        match self.calculated {
            Some(s) => Ok(s),
            None => {
                let mut result = self.raw;
                if let Some(w) = self.weapon {
                    result *= 1.0 + w.power;
                }
                result *= if self.crit_override
                    || self
                        .weapon
                        .is_some_and(|el| random::<f32>() < el.crit_chance)
                {
                    CRIT_MULTIPLIER
                } else {
                    1.0
                };
                Ok(result)
            }
        }
    }
}
