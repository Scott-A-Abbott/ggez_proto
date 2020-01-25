use ggez::graphics::Rect;
use ggez::mint::Vector2;
use specs::*;

#[derive(Component)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: Vector2<f32>,
}

impl Camera {
    pub fn new(x: f32, y: f32, width: f32, height: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            scale: Vector2 { x: scale, y: scale },
        }
    }

    pub fn to_rect(&self) -> Rect {
        self.into()
    }

    pub fn clone_from(cam: &Camera) -> Self {
        Self {
            x: cam.x,
            y: cam.y,
            width: cam.width,
            height: cam.height,
            scale: cam.scale,
        }
    }
}

impl From<Rect> for Camera {
    fn from(rect: Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.w,
            height: rect.h,
            ..Camera::default()
        }
    }
}

impl From<&Rect> for Camera {
    fn from(rect: &Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.w,
            height: rect.h,
            ..Camera::default()
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0., 0., 800., 600., 1.)
    }
}

impl From<&Camera> for Rect {
    fn from(cam: &Camera) -> Rect {
        Rect::new(cam.x, cam.y, cam.width, cam.height)
    }
}
