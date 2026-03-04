use crate::util::vec2::{Vec2, Vec2i};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2i {
        match self {
            Self::Right => Vec2::right(),
            Self::Left => Vec2::left(),
            Self::Up => Vec2::up(),
            Self::Down => Vec2::down()
        }
    }
}