use ggez::{
    conf::{Conf, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use proto::Game;
use std::{env, path};

fn main() -> GameResult {
    let conf = Conf {
        window_mode: WindowMode {
            width: 1440.0,
            height: 810.0,
            resizable: true,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: String::from("testing"),
            ..WindowSetup::default()
        },
        ..Conf::default()
    };

    let mut path = path::PathBuf::new();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
    }

    let (mut ctx, mut game_loop) = ContextBuilder::new("proto", "MetaStory Games")
        .conf(conf)
        .add_resource_path(path)
        .build()?;

    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut game_loop, &mut game)
}
