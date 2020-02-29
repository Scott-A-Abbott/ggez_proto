use super::{
    components::*,
    systems::{MeshRenderSystem, MoveSystem},
    Camera,
};
use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods, MouseButton},
    graphics,
    graphics::{Mesh, Text},
    timer, Context, GameResult,
};
use specs::{Builder, Entities, Join, ReadStorage, RunNow, WorldExt, WriteStorage};
use std::collections::HashMap;

pub struct Game {
    entity_manager: specs::World,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Game> {
        let mut entity_manager = specs::World::new();

        entity_manager.register::<Renderable<Mesh>>();
        entity_manager.register::<Doors>();
        entity_manager.register::<Camera>();
        entity_manager.register::<Player>();
        entity_manager.register::<SpecialRoom>();
        entity_manager.register::<IntentToMove>();
        entity_manager.register::<Facing>();
        entity_manager.register::<Size>();

        let screen = graphics::screen_coordinates(ctx);
        let camera = Camera::new(0.0, 0.0, screen.w, screen.h, 1.0);

        let _main_cam = entity_manager
            .create_entity()
            .with(Camera::clone_from(&camera))
            .build();

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
                        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                    )
                    .build(ctx)?,
                cur_pos: Position::new(screen.w / 2.0 + 20.0, 0.0),
                prev_pos: None
            })
            //doors should probably be a seperate entity
            .with(Doors(doors))
            .build();

        let (pw, ph) = (30.0, 30.0);
        let _player = entity_manager
            .create_entity()
            .with(Player)
            .with(Size::new(pw, ph))
            .with(Renderable {
                drawable: graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, pw, ph),
                        graphics::Color::new(0.0, 0.0, 0.0, 1.0),
                    )
                    .build(ctx)?,
                // pos: Position::new(screen.w / 2.0 - 15.0, screen.h * 0.8 - 15.0),
                cur_pos: Position::new(0.0, 0.0),
                prev_pos: None
            })
            .with(Facing {
                direction: Direction::Right,
            })
            .build();

        entity_manager.insert(camera);
        Ok(Self { entity_manager })
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

            let mut move_system = MoveSystem;
            move_system.run_now(&self.entity_manager);

            let mut cam = self.entity_manager.write_resource::<Camera>();
            //unofficial camera controlls for testing:
            for key in keycodes.iter().cloned() {
                let scale_by = 1.01;
                let max_clamp = 2.5;
                let min_clamp = 0.25;
                if key == KeyCode::Equals {
                    cam.scale.x *= scale_by;
                    cam.scale.y *= scale_by;

                    if cam.scale.x > max_clamp {
                        cam.scale.x = max_clamp;
                        cam.scale.y = max_clamp;
                    }
                }
                if key == KeyCode::Minus {
                    cam.scale.x /= scale_by;
                    cam.scale.y /= scale_by;

                    if cam.scale.x < min_clamp {
                        cam.scale.x = min_clamp;
                        cam.scale.y = min_clamp;
                    }
                }
                if key == KeyCode::Key0 {
                    cam.scale.x = 1.0;
                    cam.scale.y = 1.0;
                    cam.x = 0.0;
                    cam.y = 0.0;
                }

                let speed = 5.0;
                if key == KeyCode::A {
                    cam.x -= speed;
                }
                if key == KeyCode::D {
                    cam.x += speed;
                }
                if key == KeyCode::S {
                    cam.y += speed;
                }
                if key == KeyCode::W {
                    cam.y -= speed;
                }
            } //end camera controlls

            let (entities, mut facings, mut int_moves, players): (
                Entities,
                WriteStorage<Facing>,
                WriteStorage<IntentToMove>,
                ReadStorage<Player>,
            ) = self.entity_manager.system_data();

            if keycodes.contains(&KeyCode::Right) && keycodes.contains(&KeyCode::Left) {
                for (e, _p) in (&entities, &players).join() {
                    int_moves.remove(e);
                }
            } else {
                for key in keycodes.iter().cloned() {
                    match key {
                        KeyCode::Right => {
                            for (e, _p) in (&entities, &players).join() {
                                facings
                                    .insert(
                                        e,
                                        Facing {
                                            direction: Direction::Right,
                                        },
                                    )
                                    .expect("Player facing right");
                                int_moves
                                    .insert(e, IntentToMove)
                                    .expect("Player intent to move right");
                            }
                        }
                        KeyCode::Left => {
                            for (e, _p) in (&entities, &players).join() {
                                facings
                                    .insert(
                                        e,
                                        Facing {
                                            direction: Direction::Left,
                                        },
                                    )
                                    .expect("Player facing left");
                                int_moves
                                    .insert(e, IntentToMove)
                                    .expect("Player intent to move left");
                            }
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

        let alpha = timer::remaining_update_time(ctx);
        let mut render_system = MeshRenderSystem::new(ctx, timer::duration_to_f64(alpha));
        render_system.run_now(&self.entity_manager);

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
            let (entities, facings, mut int_move, players): (
                Entities,
                ReadStorage<Facing>,
                WriteStorage<IntentToMove>,
                ReadStorage<Player>,
            ) = self.entity_manager.system_data();

            if keycode == KeyCode::Right {
                for (e, f, _p) in (&entities, &facings, &players).join() {
                    if int_move.contains(e) && f.direction == Direction::Right {
                        int_move.remove(e);
                    }
                }
            }
            if keycode == KeyCode::Left {
                for (e, f, _p) in (&entities, &facings, &players).join() {
                    if int_move.contains(e) && f.direction == Direction::Left {
                        int_move.remove(e);
                    }
                }
            }
        }
    }
    // A keyboard button was released.

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {}
}
