use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }
    pub fn set(&mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;

        *self
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.x).sqrt()
    }
}
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;

        Self {x, y}
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;

        Self {x, y}
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

#[derive(Debug)]
pub struct mat22 {
    col1: Vec2,
    col2: Vec2
}
impl mat22 {
    pub fn new(col1: Vec2, col2: Vec2) -> Self {
        Self {col1, col2}
    }
    pub fn new_by_angle(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let col1 = Vec2::new(c, s);
        let col2 = Vec2::new(-s, c);

        Self {col1, col2}
    }
    pub fn transpose(&self) -> Self {
        let col1 = Vec2::new(self.col1.x, self.col2.x);
        let col2 = Vec2::new(self.col1.y, self.col2.y);

        Self {col1, col2}
    }
    pub fn invert(&self) -> Result<Self, String> {
        let (a, b, c, d) = (self.col1.x, self.col2.x, self.col1.y, self.col2.y);
        let det = a * b - b * c;
        
        if det == 0.0 {
            Err(format!("Could not invert: {:?}", self))
        } else {
            let det = 1.0/det;
            let col1 = Vec2::new(det*d, -det*c);
            let col2 = Vec2::new(-det*b, det*a);
            Ok(Self {col1, col2})
        }
    }
}