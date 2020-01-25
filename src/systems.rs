use crate::camera::Camera;
use crate::components::*;
use ggez::nalgebra as na;
use ggez::*;
use specs::*;

pub struct RenderSystem<'a> {
    ctx: &'a mut Context,
}
impl<'a> RenderSystem<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (cam, renderables, positions): Self::SystemData) {
        for (ren, pos) in (&renderables, &positions).join() {
            let half_width = cam.width / 2.;
            let half_height = cam.height / 2.;
            let x_offset = cam.x * cam.scale.x - half_width;
            let y_offset = cam.y * cam.scale.y - half_height;
            let x = (pos.x - half_width) * cam.scale.x;
            let y = (pos.y - half_height) * cam.scale.y;
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