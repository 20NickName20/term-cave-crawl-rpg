use crossterm::style::{Color, StyledContent, Stylize};

use crate::world::world::TargetInfo;

#[derive(Clone)]
pub enum Tile {
    Null,
    
    Floor,
    Wall,
    Vent,

    RockFloor,
    RockWall,

    Water,
    Lava,

    Door(bool),
    Tunnel(Option<TargetInfo>),
    StairUp(Option<TargetInfo>),
    StairDown(Option<TargetInfo>),
}

impl Tile {
    pub fn repr(&self) -> StyledContent<&str> {
        match self {
            Self::Floor => " .".with(Color::AnsiValue(68)),
            Self::Wall => "##".on(Color::AnsiValue(8)),
            Self::Vent => "=̅=̅".bold(),

            Self::RockFloor => " ,".with(Color::AnsiValue(130)),
            Self::RockWall => "//".on(Color::AnsiValue(130)),

            Self::Water => ",~".on(Color::AnsiValue(39)),
            Self::Lava => "/~".with(Color::AnsiValue(11)).on(Color::AnsiValue(202)),

            Self::Door(is_open) => {
                let v = if *is_open {"|+"} else {"||"};
                v.bold()
            },
            Self::StairDown(_) => ">>".on(Color::AnsiValue(233)).bold(),
            Self::StairUp(_) => "<<".on(Color::AnsiValue(233)).bold(),
            Self::Tunnel(_) => "><".on(Color::AnsiValue(233)).bold(),
            Self::Null => "^@".cyan().on_dark_red()
        }
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Wall => false,
            Self::RockWall => false,
            Self::Water => false,
            Self::Lava => false,
            Self::Door(is_open) => *is_open,
            _ => true
        }
    }

    pub fn get_target_info(&self) -> Result<Option<TargetInfo>, String> {
        match self {
            Tile::Tunnel(info) |
            Tile::StairDown(info) |
            Tile::StairUp(info) => Ok(info.clone()), // Option<TargetInfo>
            _ => Err("Not a warp tile".to_string())
        }
    }

    pub fn set_target_info(&mut self, target_info: Option<TargetInfo>) {
        match self {
            Tile::Tunnel(opt) |
            Tile::StairDown(opt) |
            Tile::StairUp(opt) => *opt = target_info,
            _ => (),
        }
    }

    pub fn get_level_delta(&self) -> Option<i32> {
        match self {
            Self::Tunnel(_) => Some(0),
            Self::StairDown(_) => Some(1),
            Self::StairUp(_) => Some(-1),
            _ => None
        }
    }

    pub fn warp_from_direction(direction: i32, target_info: Option<TargetInfo>) -> Tile {
        match direction {
            -1 => Self::StairUp(target_info),
            1 => Self::StairDown(target_info),
            _ => Self::Tunnel(target_info)
        }
    }
}