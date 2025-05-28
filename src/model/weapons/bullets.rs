use macroquad::prelude::*;

pub struct Bullet {
    damage: f32,
    knockback: f32,
    speed: Vec2,
    penetrate: bool,
    lifetime: f32,
    size: f32,
}

pub struct Quad([Vec2; 2]);

//TODO make swept rec and bounds check for collision.
