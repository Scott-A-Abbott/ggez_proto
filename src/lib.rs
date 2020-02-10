mod components;
pub use self::components::*;
mod camera;
pub use self::camera::Camera;
mod systems;
use systems::*;

use ggez::{event::*, graphics, Context, GameResult};
use specs::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Game> {
        let mut world = World::new();

        world.register::<Renderable>();
        world.register::<Doors>();
        world.register::<Camera>();
        world.register::<Player>();
        world.register::<SpecialRoom>();
        world.register::<IntentToMove>();
        world.register::<Facing>();

        let screen = graphics::screen_coordinates(ctx);
        let camera = Camera::new(0., 0., screen.w, screen.h, 1.);

        let _main_cam = world
            .create_entity()
            .with(Camera::clone_from(&camera))
            .build();

        let _start_room = world
            .create_entity()
            .with(Renderable {
                mesh: graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0., 0., screen.w * 2., screen.h),
                        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                    )
                    .build(ctx)?,
                pos: Position::new(0., 0.),
            })
            //doors should probably be a seperate entity
            .with(Doors {
                types: HashSet::from_iter(vec![DoorType::Right]),
                locations: vec![Position::new(0., screen.h - 100.)],
            })
            .build();

        let _player = world
            .create_entity()
            .with(Player)
            .with(Renderable {
                mesh: graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0., 0., 30., 30.),
                        graphics::Color::new(0.0, 0.0, 0.0, 1.0),
                    )
                    .build(ctx)?,
                pos: Position::new(screen.w / 2. - 15., screen.h * 0.8 - 15.),
            })
            .with(Facing {
                direction: Direction::Right,
            })
            .build();

        world.insert(camera);
        Ok(Self { world })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.world.maintain();
        let mut move_system = MoveSystem;
        move_system.run_now(&self.world);

        // let mut cam = self.world.write_resource::<Camera>();
        // let speed = 5.;

        let keycodes = ggez::input::keyboard::pressed_keys(ctx);

        let (entities, mut facings, mut int_moves, players): (
            Entities,
            WriteStorage<Facing>,
            WriteStorage<IntentToMove>,
            ReadStorage<Player>,
        ) = self.world.system_data();

        if keycodes.contains(&KeyCode::Right) && keycodes.contains(&KeyCode::Left) {
            for (e, _p) in (&entities, &players).join() {
                int_moves.remove(e);
            }
        } else {
            for key in keycodes.iter().cloned() {
                //     if key == KeyCode::Equals {
                //         cam.scale.x *= 1.01;
                //         cam.scale.y *= 1.01;

                //         if cam.scale.x > 2.5 {
                //             cam.scale.x = 2.5;
                //             cam.scale.y = 2.5;
                //         }
                //     }
                //     if key == KeyCode::Minus {
                //         cam.scale.x /= 1.01;
                //         cam.scale.y /= 1.01;

                //         if cam.scale.x < 0.25 {
                //             cam.scale.x = 0.25;
                //             cam.scale.y = 0.25;
                //         }
                //     }
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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let mut render_system = RenderSystem::new(ctx);
        render_system.run_now(&self.world);

        graphics::present(ctx)
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
            quit(ctx);
        }

        if keycode == KeyCode::Right || keycode == KeyCode::Left {
            let (entities, facings, mut int_move, players): (
                Entities,
                ReadStorage<Facing>,
                WriteStorage<IntentToMove>,
                ReadStorage<Player>,
            ) = self.world.system_data();

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
