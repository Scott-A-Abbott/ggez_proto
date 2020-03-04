use super::{components::*, Camera};
use ggez::graphics::{self, Mesh};
use ggez::Context;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct MeshRenderSystem<'a> {
    ctx: &'a mut Context,
    alpha: f64,
    cam: specs::Entity,
}
impl<'a> MeshRenderSystem<'a> {
    pub fn new(ctx: &'a mut Context, alpha: f64, cam: specs::Entity) -> Self {
        Self { ctx, alpha, cam }
    }
}

impl<'a> System<'a> for MeshRenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Renderable<Mesh>>,
        ReadStorage<'a, Size>,
    );

    fn run(&mut self, (cams, renderables, sizes): Self::SystemData) {
        let cam = cams.get(self.cam).expect("Could not retrieve main camera!");
        for (ren, size) in (&renderables, &sizes).join() {
            let (mut pos_x, mut pos_y) = (ren.cur_pos.x as f64, ren.cur_pos.y as f64);

            if let Some(prev_pos) = ren.prev_pos {
                let cur_ax = pos_x * self.alpha;
                let cur_ay = pos_y * self.alpha;
                let prev_ax = prev_pos.x as f64 * (1.0 - self.alpha);
                let prev_ay = prev_pos.y as f64 * (1.0 - self.alpha);

                pos_x = cur_ax + prev_ax;
                pos_y = cur_ay + prev_ay;
            }

            let (mut cam_x, mut cam_y) = (cam.cur_pos.x as f64, cam.cur_pos.y as f64);

            if let Some(cam_prev_pos) = cam.prev_pos {
                let cur_ax = cam_x * self.alpha;
                let cur_ay = cam_y * self.alpha;
                let prev_ax = cam_prev_pos.x as f64 * (1.0 - self.alpha);
                let prev_ay = cam_prev_pos.y as f64 * (1.0 - self.alpha);

                cam_x = cur_ax + prev_ax;
                cam_y = cur_ay + prev_ay;
            }

            let x_offset = (cam_x as f32 * cam.cur_scale.x) - (cam.width / 2.0);
            let y_offset = (cam_y as f32 * cam.cur_scale.y) - (cam.height / 2.0);

            let center_x = (pos_x as f32 - size.width / 2.0) * cam.cur_scale.x;
            let center_y = (-pos_y as f32 - size.height / 2.0) * cam.cur_scale.y;

            let x = center_x - x_offset;
            let y = center_y - y_offset;

            graphics::draw(
                &mut self.ctx,
                &ren.drawable,
                graphics::DrawParam::default()
                    .dest(Position::new(x, y))
                    .scale(cam.cur_scale),
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

pub struct StopMovingSystem;
impl<'a> System<'a> for StopMovingSystem {
    type SystemData = (
        WriteStorage<'a, Renderable<Mesh>>,
        ReadStorage<'a, IntentToMove>,
    );

    fn run(&mut self, (mut renderables, int_moves): Self::SystemData) {
        for (ren, im) in (&mut renderables, (&int_moves).maybe()).join() {
            if im.is_none() && ren.prev_pos.is_some() {
                ren.prev_pos = None;
            }
        }
    }
}

pub struct MoveCamSystem;
impl<'a> System<'a> for MoveCamSystem {
    type SystemData = (WriteStorage<'a, Camera>, ReadStorage<'a, IntentToMove>);

    fn run(&mut self, (mut cams, int_moves): Self::SystemData) {
        const SPEED: f32 = 5.0;
        for (cam, moves) in (&mut cams, &int_moves).join() {
            let IntentToMove(moves) = moves;
            cam.prev_pos = Some(cam.cur_pos);
            for m in moves.iter() {
                use Direction::*;
                match m {
                    Up => {
                        cam.cur_pos.y -= SPEED;
                    }
                    Down => {
                        cam.cur_pos.y += SPEED;
                    }
                    Left => {
                        cam.cur_pos.x -= SPEED;
                    }
                    Right => {
                        cam.cur_pos.x += SPEED;
                    }
                }
            }
        }
    }
}
