use ggez::graphics::Drawable;
use specs::{Component, DenseVecStorage, Entity};
use std::collections::{HashMap, HashSet};

pub struct Door {
    pub to_room: Entity,
    pub pos: Position,
}

#[derive(PartialEq, Eq, Hash)]
pub enum DoorType {
    Right,
    Left,
    Middle,
    Top,
    Bottom,
}

#[derive(Component)]
pub struct Doors(pub HashMap<DoorType, Door>);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
#[derive(Component)]
pub struct Facing {
    pub direction: Direction,
}

#[derive(Component)]
pub struct IntentToMove(pub HashSet<Direction>);

#[derive(Component)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl From<ggez::mint::Point2<f32>> for Position {
    fn from(point: ggez::mint::Point2<f32>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
impl From<Position> for ggez::mint::Point2<f32> {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

#[derive(Component)]
pub struct Renderable<D>
where
    D: Drawable + Send + Sync + 'static,
{
    pub drawable: D,
    pub cur_pos: Position,
    pub prev_pos: Option<Position>,
    pub draw_param: Option<ggez::graphics::DrawParam>,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
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
pub struct Target {
    entity: Entity,
    offset: f32,
}
