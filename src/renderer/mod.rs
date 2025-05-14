use camera::{UICamera, WorldCamera};

use crate::model::{Status, World};

mod camera;
mod ui;
mod world;

pub struct Renderer {
    wcamera: WorldCamera,
    uicamera: UICamera,
}
impl Renderer {
    pub async fn render_ui(&mut self, stat: &Status) {
        self.uicamera.activate();
        ui::render_ui(stat).await
    }

    pub async fn render_world(&mut self, world: &World) {
        self.wcamera.activate();
        world::render_world(world).await
    }

    pub fn new() -> Renderer {
        Renderer {
            wcamera: WorldCamera::new(macroquad::camera::Camera2D::default()),
            uicamera: UICamera::new(),
        }
    }
}
