use super::*;
use ggez::Context;
use specs::{Join, Read, ReadStorage, System};

pub struct MeshRenderSystem<'a> {
    ctx: &'a mut Context,
}
impl<'a> MeshRenderSystem<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> System<'a> for MeshRenderSystem<'a> {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Renderable<Mesh>>,
        ReadStorage<'a, Size>,
    );

    fn run(&mut self, (cam, renderables, sizes): Self::SystemData) {
        for (ren, size) in (&renderables, &sizes).join() {
            let x_offset = (cam.x * cam.scale.x) - (cam.width / 2.0);
            let y_offset = (cam.y * cam.scale.y) - (cam.height / 2.0);

            let center_x = (ren.pos.x - size.width / 2.0) * cam.scale.x;
            let center_y = (-ren.pos.y - size.height / 2.0) * cam.scale.y;

            let x = center_x - x_offset;
            let y = center_y - y_offset;

            graphics::draw(
                &mut self.ctx,
                &ren.drawable,
                graphics::DrawParam::default()
                    .dest(Position::new(x, y))
                    .scale(cam.scale)
                    .offset(Position::new(1., 1.)),
            )
            .expect("Drawing a renderable");
        }
    }
}

pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        // Read<'a, DeltaTime>,
        ReadStorage<'a, Facing>,
        ReadStorage<'a, IntentToMove>,
        WriteStorage<'a, Renderable<Mesh>>,
    );

    fn run(&mut self, (facings, intentions, mut renderables): Self::SystemData) {
        // use std::ops::Deref;
        for (facing, _move_int, ren) in (&facings, &intentions, &mut renderables).join() {
            if facing.direction == Direction::Right {
                // ren.pos.x += 250.0 * dt.deref();
                ren.pos.x += 2.5;
            }
            if facing.direction == Direction::Left {
                // ren.pos.x -= 250.0 * dt.deref();
                ren.pos.x -= 2.5;
            }
        }
    }
}
