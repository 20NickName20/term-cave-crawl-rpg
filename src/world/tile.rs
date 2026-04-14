use crossterm::style::{Color, StyledContent, Stylize};

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Vent,

    Grass,
    Tree,

    RockFloor,
    RockWall,

    Water,
    Lava,

    Door(bool),
    Tunnel,
    StairUp,
    StairDown,
}

impl Tile {
    pub fn repr(&self) -> StyledContent<&str> {
        match self {
            Self::Floor => " .".with(Color::AnsiValue(68)),
            Self::Wall => "##".on(Color::AnsiValue(8)),
            Self::Vent => "=̅=̅".bold(),

            Self::Grass => " ,".with(Color::AnsiValue(70)),
            Self::Tree => " 7".with(Color::AnsiValue(2)),

            Self::RockFloor => ",'".with(Color::AnsiValue(178)),
            Self::RockWall => "//".on(Color::AnsiValue(130)),

            Self::Water => ",~".on(Color::AnsiValue(39)),
            Self::Lava => "/~".with(Color::AnsiValue(11)).on(Color::AnsiValue(202)),

            Self::Door(is_open) => {
                let v = if *is_open {"|+"} else {"||"};
                v.bold()
            },
            Self::StairDown => ">>".on(Color::AnsiValue(233)).bold(),
            Self::StairUp => "<<".on(Color::AnsiValue(233)).bold(),
            Self::Tunnel => "><".on(Color::AnsiValue(233)).bold()
        }
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Self::Wall => false,
            Self::RockWall => false,
            Self::Tree => false,
            Self::Water => false,
            Self::Lava => false,
            Self::Door(is_open) => *is_open,
            _ => true
        }
    }

    pub fn get_level_delta(&self) -> Option<i32> {
        match self {
            Self::Tunnel => Some(0),
            Self::StairDown => Some(1),
            Self::StairUp => Some(-1),
            _ => None
        }
    }

    pub fn warp_from_direction(direction: i32) -> Tile {
        match direction {
            -1 => Self::StairUp,
            1 => Self::StairDown,
            _ => Self::Tunnel
        }
    }
}
