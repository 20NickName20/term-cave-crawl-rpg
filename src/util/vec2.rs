use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

use crate::util::direction::Direction;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    pub fn from(direction: Direction) -> Self { direction.to_vec2() }

    pub fn x_u16(&self) -> u16 {
        self.x as u16
    }
    pub fn y_u16(&self) -> u16 {
        self.y as u16
    }

    pub fn left() -> Self { Self { x: -1, y: 0 } }
    pub fn right() -> Self { Self { x: 1, y: 0 } }
    pub fn up() -> Self { Self { x: 0, y: -1 } }
    pub fn down() -> Self { Self { x: 0, y: 1 } }
    pub fn add(mut self, x: i32, y: i32) -> Self {
        self.x += x;
        self.y += y;
        return self;
    }
    pub fn sub(mut self, x: i32, y: i32) -> Self {
        self.x -= x;
        self.y -= y;
        return self;
    }
    pub fn scale(mut self, s: i32) -> Self {
        self.x *= s;
        self.y *= s;
        return self;
    }
}

impl ToString for Vec2 {
    fn to_string(&self) -> String {
        format!("Vec2({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y) }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y) }
}
impl Mul<i32> for Vec2 {
    type Output = Self;
    fn mul(self, other: i32) -> Self { Self::new(self.x * other, self.y * other) }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}