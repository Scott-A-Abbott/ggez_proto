use ggez::{
    conf::{Conf, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use proto::Game;

fn main() -> GameResult {
    let conf = Conf {
        window_mode: WindowMode {
            // width: 1920.0,
            // height: 1080.0,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: "proto".to_owned(),
            ..WindowSetup::default()
        },
        ..Conf::default()
    };

    let (mut ctx, mut game_loop) = ContextBuilder::new("proto", "Metastory Games")
        .conf(conf)
        .build()?;

    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut game_loop, &mut game)
}
