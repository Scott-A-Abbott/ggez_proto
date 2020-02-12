use ggez::{
    conf::{Conf, WindowSetup},
    event, ContextBuilder, GameResult,
};
use proto::Game;

fn main() -> GameResult {
    let conf = Conf {
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
