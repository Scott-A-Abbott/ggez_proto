use super::{
    components::*,
    systems::{MoveSystem, RenderSystem, StopMovingSystem},
    Camera,
};
use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods, MouseButton},
    graphics,
    graphics::{Image, Mesh, Text},
    timer, Context, GameResult,
};
use specs::{Builder, Entities, Entity, Join, ReadStorage, RunNow, WorldExt, WriteStorage};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

pub struct Game {
    entity_manager: specs::World,
    main_cam: Entity,
    player: Entity,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Game> {
        let mut entity_manager = specs::World::new();

        entity_manager.register::<Renderable<Mesh>>();
        entity_manager.register::<Renderable<Image>>();
        entity_manager.register::<Doors>();
        entity_manager.register::<Camera>();
        entity_manager.register::<Player>();
        entity_manager.register::<SpecialRoom>();
        entity_manager.register::<IntentToMove>();
        entity_manager.register::<Facing>();
        entity_manager.register::<Size>();

        let screen = graphics::screen_coordinates(ctx);
        let camera = Camera::new(Position::new(0.0, 0.0), screen.w, screen.h, 1.0);

        let main_cam = entity_manager.create_entity().with(camera).build();

        let next_room = entity_manager.create_entity().build();
        let mut doors = HashMap::new();
        doors.insert(
            DoorType::Right,
            Door {
                to_room: next_room,
                pos: Position::new(0.0, screen.w - 50.0),
            },
        );
        let (stw, sth) = (screen.w * 2.0, screen.h - 40.0);
        let _start_room = entity_manager
            .create_entity()
            .with(Size::new(stw, sth))
            .with(Renderable {
                drawable: graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, stw, sth),
                        graphics::Color::new(0.0, 0.0, 1.0, 1.0),
                    )
                    .build(ctx)?,
                cur_pos: Position::new(screen.w / 2.0 + 20.0, 0.0),
                prev_pos: None,
                draw_param: None,
            })
            //doors should probably be a seperate entity
            .with(Doors(doors))
            .build();

        let (pw, ph) = (300.0, 400.0);
        let player = entity_manager
            .create_entity()
            .with(Player)
            .with(Size::new(pw, ph))
            .with(Renderable {
                drawable: graphics::Image::new(ctx, "/anim_tmp.png")?,

                // drawable: graphics::MeshBuilder::new()
                //     .rectangle(
                //         graphics::DrawMode::fill(),
                //         graphics::Rect::new(0.0, 0.0, pw, ph),
                //         graphics::Color::new(0.0, 0.0, 0.0, 1.0),
                //     )
                //     .build(ctx)?,
                // pos: Position::new(screen.w / 2.0 - 15.0, screen.h * 0.8 - 15.0),
                cur_pos: Position::new(0.0, 0.0),
                prev_pos: None,
                draw_param: Some(
                    graphics::DrawParam::new().src(graphics::Rect::new(0.0, 0.0, 0.5, 0.5)),
                ),
            })
            .with(Facing {
                direction: Direction::Right,
            })
            .build();

        Ok(Self {
            entity_manager,
            main_cam,
            player,
        })
    }
}

pub struct DeltaTime(f32);
impl Default for DeltaTime {
    fn default() -> Self {
        Self(0.0)
    }
}
impl std::ops::Mul<&DeltaTime> for f32 {
    type Output = Self;

    fn mul(self, other: &DeltaTime) -> f32 {
        let DeltaTime(dt) = other;
        self * dt
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.entity_manager.maintain();

        const DESIRED_FPS: u32 = 73;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let keycodes = ggez::input::keyboard::pressed_keys(ctx);

            // let delta_time = 1.0 / DESIRED_FPS as f32;
            // self.entity_manager.insert(DeltaTime(delta_time));

            let mut mv_mesh_sys = MoveSystem::<Mesh>::new();
            mv_mesh_sys.run_now(&self.entity_manager);

            let mut mv_img_sys = MoveSystem::<Image>::new();
            mv_img_sys.run_now(&self.entity_manager);

            let mut move_cam_system = super::systems::MoveCamSystem;
            move_cam_system.run_now(&self.entity_manager);

            let mut stp_mesh_sys = StopMovingSystem::<Mesh>::new();
            stp_mesh_sys.run_now(&self.entity_manager);

            let mut stp_img_sys = StopMovingSystem::<Image>::new();
            stp_img_sys.run_now(&self.entity_manager);

            let (mut facings, mut int_moves, mut cams): (
                WriteStorage<Facing>,
                WriteStorage<IntentToMove>,
                WriteStorage<Camera>,
            ) = self.entity_manager.system_data();
            let mut cam = cams.get_mut(self.main_cam).unwrap();
            //unofficial camera controlls for testing:

            fn add_move_dir_for_cam(
                dir: Direction,
                int_moves: &mut WriteStorage<IntentToMove>,
                main_cam: Entity,
            ) {
                if int_moves.contains(main_cam) {
                    let IntentToMove(moves) = int_moves.get(main_cam).unwrap();
                    let mut new_moves = HashSet::new();
                    for m in moves {
                        new_moves.insert(*m);
                    }
                    new_moves.insert(dir);
                    int_moves
                        .insert(main_cam, IntentToMove(new_moves))
                        .expect("Could not replace IntentToMove for main_cam");
                } else {
                    int_moves
                        .insert(main_cam, IntentToMove(HashSet::from_iter(vec![dir])))
                        .expect("Could not insert IntentToMove for main_cam");
                }
            }
            for key in keycodes.iter().cloned() {
                if key == KeyCode::Key0 {
                    cam.cur_scale.x = 1.0;
                    cam.cur_scale.y = 1.0;
                    cam.cur_pos.x = 0.0;
                    cam.cur_pos.y = 0.0;
                }

                match key {
                    KeyCode::A => {
                        add_move_dir_for_cam(Direction::Left, &mut int_moves, self.main_cam)
                    }
                    KeyCode::D => {
                        add_move_dir_for_cam(Direction::Right, &mut int_moves, self.main_cam)
                    }
                    KeyCode::S => {
                        add_move_dir_for_cam(Direction::Down, &mut int_moves, self.main_cam)
                    }
                    KeyCode::W => {
                        add_move_dir_for_cam(Direction::Up, &mut int_moves, self.main_cam)
                    }
                    _ => {}
                }
            } //end camera controlls

            if !keycodes.contains(&KeyCode::A)
                && !keycodes.contains(&KeyCode::D)
                && !keycodes.contains(&KeyCode::S)
                && !keycodes.contains(&KeyCode::W)
            {
                int_moves.remove(self.main_cam);
                cam.prev_pos = None;
            }

            if keycodes.contains(&KeyCode::Right) && keycodes.contains(&KeyCode::Left) {
                int_moves.remove(self.player);
            } else {
                for key in keycodes.iter().cloned() {
                    match key {
                        KeyCode::Right => {
                            facings
                                .insert(
                                    self.player,
                                    Facing {
                                        direction: Direction::Right,
                                    },
                                )
                                .expect("Player facing right");
                            int_moves
                                .insert(
                                    self.player,
                                    IntentToMove(HashSet::from_iter(vec![Direction::Right])),
                                )
                                .expect("Player intent to move right");
                        }
                        KeyCode::Left => {
                            facings
                                .insert(
                                    self.player,
                                    Facing {
                                        direction: Direction::Left,
                                    },
                                )
                                .expect("Player facing left");
                            int_moves
                                .insert(
                                    self.player,
                                    IntentToMove(HashSet::from_iter(vec![Direction::Left])),
                                )
                                .expect("Player intent to move left");
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let tr = timer::remaining_update_time(ctx);
        let dt: f64 = 1.0 / 73.0;
        let alpha = timer::duration_to_f64(tr) / dt;
        {
            let mut mesh_render_system = RenderSystem::<Mesh>::new(ctx, alpha, self.main_cam);
            mesh_render_system.run_now(&self.entity_manager);
        }
        {
            let mut img_render_system = RenderSystem::<Image>::new(ctx, alpha, self.main_cam);
            img_render_system.run_now(&self.entity_manager);
        }

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps));
        graphics::draw(
            ctx,
            &fps_display,
            (Position::new(50.0, 0.0), graphics::WHITE),
        )?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    //    A mouse button was pressed

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    // A mouse button was released

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}
    // The mouse was moved; it provides both absolute x and y coordinates in the window, and relative x and y coordinates compared to its last position.

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    // The mousewheel was scrolled, vertically (y, positive away from and negative toward the user) or horizontally (x, positive to the right and negative to the left).

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }
    // A keyboard button was pressed.

    // The default implementation of this will call ggez::event::quit() when the escape key is pressed. If you override this with your own event handler you have to re-implment that functionality yourself.

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }

        if keycode == KeyCode::Right || keycode == KeyCode::Left {
            let (facings, mut int_move): (ReadStorage<Facing>, WriteStorage<IntentToMove>) =
                self.entity_manager.system_data();
            let f = facings.get(self.player).unwrap();

            if keycode == KeyCode::Right {
                if int_move.contains(self.player) && f.direction == Direction::Right {
                    int_move.remove(self.player);
                }
            }
            if keycode == KeyCode::Left {
                if int_move.contains(self.player) && f.direction == Direction::Left {
                    int_move.remove(self.player);
                }
            }
        }

        if keycode == KeyCode::A
            || keycode == KeyCode::W
            || keycode == KeyCode::S
            || keycode == KeyCode::D
        {
            let (mut int_move, cams): (WriteStorage<IntentToMove>, ReadStorage<Camera>) =
                self.entity_manager.system_data();
            for (im, _c) in (&mut int_move, &cams).join() {
                let IntentToMove(moves) = im;
                if keycode == KeyCode::A {
                    moves.remove(&Direction::Left);
                }
                if keycode == KeyCode::D {
                    moves.remove(&Direction::Right);
                }
                if keycode == KeyCode::W {
                    moves.remove(&Direction::Up);
                }
                if keycode == KeyCode::S {
                    moves.remove(&Direction::Down);
                }
            }
        }
    }
    // A keyboard button was released.

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {}
}
