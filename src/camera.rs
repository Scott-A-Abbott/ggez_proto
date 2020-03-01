use ggez::graphics::Rect;
use ggez::mint::Vector2;
use specs::{Component, DenseVecStorage};
use super::Position;

#[derive(Component)]
pub struct Camera {
    pub cur_pos: Position,
    pub prev_pos: Option<Position>,
    pub width: f32,
    pub height: f32,
    pub cur_scale: Vector2<f32>,
    pub prev_scale: Option<Vector2<f32>>,
}

impl Camera {
    pub fn new(cur_pos: Position, width: f32, height: f32, scale: f32) -> Self {
        Self {
            cur_pos,
            prev_pos: None,
            width,
            height,
            cur_scale: Vector2 { x: scale, y: scale },
            prev_scale: None,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Position::new(0.0, 0.0), 800., 600., 1.)
    }
}

impl From<Rect> for Camera {
    fn from(rect: Rect) -> Self {
        Self {
            cur_pos: Position::new(rect.x, rect.y),
            width: rect.w,
            height: rect.h,
            ..Camera::default()
        }
    }
}

impl From<&Rect> for Camera {
    fn from(rect: &Rect) -> Self {
        Self {
            cur_pos: Position::new(rect.x, rect.y),
            width: rect.w,
            height: rect.h,
            ..Camera::default()
        }
    }
}