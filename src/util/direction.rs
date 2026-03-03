use crate::util::vec2::Vec2;

#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Self::Right => Vec2::right(),
            Self::Left => Vec2::left(),
            Self::Up => Vec2::up(),
            Self::Down => Vec2::down()
        }
    }
}