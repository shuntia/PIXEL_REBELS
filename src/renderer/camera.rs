use std::ops::Deref;

use macroquad::{prelude::*, window};

pub struct UICamera;

impl UICamera {
    pub fn activate(&self) {
        set_default_camera();
    }
    pub fn new() -> Self {
        Self
    }
}

pub struct WorldCamera {
    camera: Camera2D,
}

impl WorldCamera {
    pub fn activate(&self) {
        set_camera(&self.camera);
    }
    pub fn new(camera: Camera2D) -> Self {
        Self { camera }
    }
}
impl Default for WorldCamera {
    fn default() -> Self {
        Self {
            camera: Camera2D {
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
        }
    }
}

impl Deref for WorldCamera {
    type Target = Camera2D;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

pub enum Rig {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Center,
}

pub fn get_scale() -> Vec2 {
    let w = window::screen_width();
    let h = window::screen_height();
    Vec2 { x: w, y: h }
}

pub fn get_origin(origin: Rig) -> Vec2 {
    match origin {
        Rig::TopLeft => vec2(0.0, 0.0),
        Rig::TopRight => vec2(window::screen_width(), 0.0),
        Rig::Center => get_scale().midpoint(vec2(0.0, 0.0)),
        Rig::BottomLeft => vec2(0.0, window::screen_height()),
        Rig::BottomRight => get_scale(),
    }
}
