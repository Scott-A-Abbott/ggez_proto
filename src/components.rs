use ggez::graphics::Mesh;
use specs::*;
use std::collections::HashSet;

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
pub struct Player;

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
    pub pos: Position,
}

pub enum RoomType {
    Boss,
    Start,
}
#[derive(Component)]
pub struct SpecialRoom {
    label: RoomType,
}

impl SpecialRoom {
    pub fn new(label: RoomType) -> Self {
        Self { label }
    }
}

#[derive(Component)]
pub struct IntentToMove;

#[derive(PartialEq)]
pub enum Direction {
    Right,
    Left,
}
#[derive(Component)]
pub struct Facing {
    pub direction: Direction,
}
