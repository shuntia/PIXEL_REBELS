pub trait Weapon {
    fn is_projectile(&self) -> bool;
    fn power(&self) -> f32;
    fn crit_chance(&self) -> f32;
}
