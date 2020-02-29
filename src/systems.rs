use super::{components::*, Camera};
use ggez::graphics::{self, Mesh};
use ggez::Context;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MeshRenderSystem<'a> {
    ctx: &'a mut Context,
    alpha: f64
}
impl<'a> MeshRenderSystem<'a> {
    pub fn new(ctx: &'a mut Context, alpha: f64) -> Self {
        Self { ctx, alpha }
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

            let (mut pos_x, mut pos_y) = (ren.cur_pos.x, ren.cur_pos.y);

            if let Some(prev_pos) = ren.prev_pos {
                let cur_ax = pos_x * self.alpha as f32;
                let cur_ay = pos_y * self.alpha as f32;
                let prev_ax = prev_pos.x * (1.0 - self.alpha as f32);
                let prev_ay = prev_pos.y * (1.0 - self.alpha as f32);

                pos_x = cur_ax + prev_ax;
                pos_y = cur_ay + prev_ay;
            }

            let x_offset = (cam.x * cam.scale.x) - (cam.width / 2.0);
            let y_offset = (cam.y * cam.scale.y) - (cam.height / 2.0);

            let center_x = (pos_x - size.width / 2.0) * cam.scale.x;
            let center_y = (-pos_y - size.height / 2.0) * cam.scale.y;

            let x = center_x - x_offset;
            let y = center_y - y_offset;

            graphics::draw(
                &mut self.ctx,
                &ren.drawable,
                graphics::DrawParam::default()
                    .dest(Position::new(x, y))
                    .scale(cam.scale)
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
                ren.prev_pos = Some(ren.cur_pos);
                ren.cur_pos.x += 2.5;
            }
            if facing.direction == Direction::Left {
                // ren.pos.x -= 250.0 * dt.deref();
                ren.prev_pos = Some(ren.cur_pos);
                ren.cur_pos.x -= 2.5;
            }
        }
    }
}
