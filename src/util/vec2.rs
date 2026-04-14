use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u16>;

/// Marker bound for scalar types that support the usual arithmetic ops.
trait Num:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Copy
{}
impl<T> Num for T
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Copy,
{}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2i {
    pub fn left() -> Self { Self { x: -1, y: 0 } }
    pub fn right() -> Self { Self { x: 1, y: 0 } }
    pub fn up() -> Self { Self { x: 0, y: -1 } }
    pub fn down() -> Self { Self { x: 0, y: 1 } }
}

impl<T: std::fmt::Display> ToString for Vec2<T> {
    fn to_string(&self) -> String {
        format!("Vec2({}, {})", self.x, self.y)
    }
}

impl<T: Num> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Num> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Num> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl<T: Num> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Num> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Conversions between signed/unsigned Vec2 variants
impl From<Vec2u> for Vec2i {
    fn from(v: Vec2u) -> Self {
        Vec2i { x: v.x as i32, y: v.y as i32 }
    }
}

impl From<Vec2i> for Vec2u {
    fn from(v: Vec2i) -> Self {
        let x = v.x.clamp(0, u16::MAX as i32) as u16;
        let y = v.y.clamp(0, u16::MAX as i32) as u16;
        Vec2u { x, y }
    }
}

// Mixed arithmetic between signed and unsigned Vec2 types.
impl std::ops::Add<Vec2i> for Vec2u {
    type Output = Vec2i;
    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i { x: self.x as i32 + other.x, y: self.y as i32 + other.y }
    }
}

impl std::ops::Add<Vec2u> for Vec2i {
    type Output = Vec2i;
    fn add(self, other: Vec2u) -> Vec2i {
        Vec2i { x: self.x + other.x as i32, y: self.y + other.y as i32 }
    }
}

impl std::ops::Sub<Vec2i> for Vec2u {
    type Output = Vec2i;
    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i { x: self.x as i32 - other.x, y: self.y as i32 - other.y }
    }
}

impl std::ops::Sub<Vec2u> for Vec2i {
    type Output = Vec2i;
    fn sub(self, other: Vec2u) -> Vec2i {
        Vec2i { x: self.x - other.x as i32, y: self.y - other.y as i32 }
    }
}
