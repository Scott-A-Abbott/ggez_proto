use specs::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DoorType {
    Right(Entity),
    Left(Entity),
    Middle(Entity),
    Top(Entity),
    Bottom(Entity),
}
#[derive(Component)]
pub struct Doors {
    pub types: std::collections::HashSet<DoorType>,
    pub locations: Vec<Position>,
}

#[derive(PartialEq)]
pub enum Direction {
    Right,
    Left,
}
#[derive(Component)]
pub struct Facing {
    pub direction: Direction,
}

#[derive(Component)]
pub struct IntentToMove;

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
    D: ggez::graphics::Drawable + Send + Sync + 'static,
{
    pub drawable: D,
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
pub struct Target {
    entity: Entity,
    offset: f32,
}
