use crate::camera::Camera;
use crate::components::*;
use ggez::event::KeyCode;
use ggez::nalgebra as na;
use ggez::*;
use specs::*;
use std::collections::HashSet;

pub struct RenderSystem<'a> {
    ctx: &'a mut Context,
}
impl<'a> RenderSystem<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (Read<'a, Camera>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (cam, renderables): Self::SystemData) {
        for ren in (&renderables).join() {
            let half_width = cam.width / 2.;
            let half_height = cam.height / 2.;
            let x_offset = cam.x * cam.scale.x - half_width;
            let y_offset = cam.y * cam.scale.y - half_height;
            let x = (ren.pos.x - half_width) * cam.scale.x;
            let y = (ren.pos.y - half_height) * cam.scale.y;
            graphics::draw(
                &mut self.ctx,
                &ren.mesh,
                graphics::DrawParam::default()
                    .dest(na::Point2::new(x - x_offset, y - y_offset))
                    .scale(cam.scale),
            )
            .unwrap();
        }
    }
}

pub struct InputSystem<'a> {
    keycodes: &'a HashSet<KeyCode>,
}
impl<'a> InputSystem<'a> {
    pub fn new(keycodes: &'a HashSet<KeyCode>) -> Self {
        Self { keycodes }
    }
}
impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Renderable>);

    fn run(&mut self, (players, mut renderables): Self::SystemData) {
        for (_p, ren) in (&players, &mut renderables).join() {
            let speed = 2.5;

            for key in self.keycodes.iter().cloned() {
                if key == KeyCode::Left {
                    ren.pos.x -= speed;
                }
                if key == KeyCode::Right {
                    ren.pos.x += speed;
                }
            }
        }
    }
}
