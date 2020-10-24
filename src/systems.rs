use super::{components::*, Camera};
use ggez::graphics::{self, Drawable};
use ggez::Context;
use specs::{Join, ReadStorage, System, WriteStorage};
use std::marker::{PhantomData, Send, Sync};

fn calc_alpha(pos_x: f32, pos_y: f32, prev_x: f32, prev_y: f32, alpha: f64) -> (f32, f32) {
    let cur_ax = (pos_x as f64) * alpha;
    let cur_ay = (pos_y as f64) * alpha;
    let prev_ax = (prev_x as f64) * (1.0 - alpha);
    let prev_ay = (prev_y as f64) * (1.0 - alpha);
    let ax_intpol = (cur_ax + prev_ax) as f32;
    let ay_intpol = (cur_ay + prev_ay) as f32;

    (ax_intpol, ay_intpol)
}

fn calc_coords<T: Drawable + Send + Sync>(
    ren: &Renderable<T>,
    size: &Size,
    cam: &Camera,
    alpha: f64,
) -> (f32, f32) {
    let (mut pos_x, mut pos_y) = (ren.cur_pos.x, ren.cur_pos.y);
    if let Some(prev_pos) = ren.prev_pos {
        let (new_x, new_y) = calc_alpha(pos_x, pos_y, prev_pos.x, prev_pos.y, alpha);
        pos_x = new_x;
        pos_y = new_y;
    }

    let (mut cam_x, mut cam_y) = (cam.cur_pos.x, cam.cur_pos.y);
    if let Some(cam_prev_pos) = cam.prev_pos {
        let (new_x, new_y) = calc_alpha(cam_x, cam_y, cam_prev_pos.x, cam_prev_pos.y, alpha);
        cam_x = new_x;
        cam_y = new_y;
    }

    let x_offset = (cam_x * cam.cur_scale.x) - (cam.width / 2.0);
    let y_offset = (cam_y * cam.cur_scale.y) - (cam.height / 2.0);

    let center_x = (pos_x - size.width / 2.0) * cam.cur_scale.x;
    let center_y = (-pos_y - size.height / 2.0) * cam.cur_scale.y;

    (center_x - x_offset, center_y - y_offset)
}

pub struct RenderSystem<'a, D>
where
    D: Drawable,
{
    ctx: &'a mut Context,
    alpha: f64,
    cam: specs::Entity,
    phantom: PhantomData<D>,
}
impl<'a, D> RenderSystem<'a, D>
where
    D: Drawable,
{
    pub fn new(ctx: &'a mut Context, alpha: f64, cam: specs::Entity) -> Self {
        Self {
            ctx,
            alpha,
            cam,
            phantom: PhantomData::<D>,
        }
    }
}
impl<'a, D> System<'a> for RenderSystem<'a, D>
where
    D: Drawable + Send + Sync + 'static,
{
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Renderable<D>>,
        ReadStorage<'a, Size>,
    );

    fn run(&mut self, (cams, renderables, sizes): Self::SystemData) {
        let cam = cams.get(self.cam).expect("Could not retrieve main camera!");
        for (ren, size) in (&renderables, &sizes).join() {
            let (x, y) = calc_coords(ren, size, cam, self.alpha);
            let mut draw_param = graphics::DrawParam::default()
                .dest(Position::new(x, y))
                .scale(cam.cur_scale);
            if let Some(dp) = ren.draw_param {
                draw_param = dp.dest(Position::new(x, y)).scale(cam.cur_scale);
            }
            graphics::draw(&mut self.ctx, &ren.drawable, draw_param).expect("Drawing a renderable");
        }
    }
}

pub struct MoveSystem<D>(PhantomData<D>)
where
    D: Drawable;
impl<D> MoveSystem<D>
where
    D: Drawable,
{
    pub fn new() -> Self {
        Self(PhantomData::<D>)
    }
}
impl<'a, D> System<'a> for MoveSystem<D>
where
    D: Drawable + Send + Sync + 'static,
{
    type SystemData = (
        // Read<'a, DeltaTime>,
        ReadStorage<'a, Facing>,
        ReadStorage<'a, IntentToMove>,
        WriteStorage<'a, Renderable<D>>,
    );

    fn run(&mut self, (facings, intentions, mut renderables): Self::SystemData) {
        // use std::ops::Deref;
        for (facing, _move_int, ren) in (&facings, &intentions, &mut renderables).join() {
            if facing.direction == Direction::Right {
                // ren.pos.x += 250.0 * dt.deref();
                ren.prev_pos = Some(ren.cur_pos);
                ren.cur_pos.x += 4.0;
            }
            if facing.direction == Direction::Left {
                // ren.pos.x -= 250.0 * dt.deref();
                ren.prev_pos = Some(ren.cur_pos);
                ren.cur_pos.x -= 4.0;
            }
        }
    }
}

pub struct StopMovingSystem<D>(PhantomData<D>)
where
    D: Drawable;
impl<D> StopMovingSystem<D>
where
    D: Drawable,
{
    pub fn new() -> Self {
        Self(PhantomData::<D>)
    }
}
impl<'a, D> System<'a> for StopMovingSystem<D>
where
    D: Drawable + Sync + Send + 'static,
{
    type SystemData = (
        WriteStorage<'a, Renderable<D>>,
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
