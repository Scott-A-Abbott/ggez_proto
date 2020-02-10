use super::*;
use ggez::{graphics, nalgebra as na, Context};
use specs::{Join, Read, ReadStorage, System};

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
            .expect("Drawing a renderable");
        }
    }
}

pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Facing>,
        ReadStorage<'a, IntentToMove>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, (facings, intentions, mut renderables): Self::SystemData) {
        for (facing, _move_int, ren) in (&facings, &intentions, &mut renderables).join() {
            if facing.direction == Direction::Right {
                ren.pos.x += 2.5;
            }
            if facing.direction == Direction::Left {
                ren.pos.x -= 2.5;
            }
        }
    }
}
