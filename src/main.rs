use ggez::{event, ContextBuilder, GameResult};
use proto::Game;

fn main() -> GameResult {
    let (mut ctx, mut game_loop) = ContextBuilder::new("proto", "Metastory Games").build()?;

    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut game_loop, &mut game)
}
