use ggez::graphics::*;
use specs::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Renderable {
    pub mesh: Mesh,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DoorType {
    Right,
    Left,
    Middle,
    Top,
    Bottom,
}
#[derive(Component)]
pub struct Doors {
    pub types: HashSet<DoorType>,
    pub locations: Vec<Position>,
}

#[derive(Component)]
pub struct StartRoom;

#[derive(Component)]
pub struct BossRoom;
