use crossterm::style::{StyledContent, Stylize};

use crate::{util::{direction::Direction, vec2::Vec2u}, world::map::Map};

pub struct Entity {
    id: u32,
    pub pos: Vec2u,
    pub data: EntityData
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Action {
    #[default]
    Idle,
    Move(Direction),
    UseWarpTile,
    Attack(Direction),
    Interact(Direction)
}

pub struct LivingData {
    pub health: u32,
    pub direction: Direction,
    pub action: Action
}

impl LivingData {
    pub fn new(health: u32, direction: Direction) -> LivingData {
        LivingData { health, direction, action: Default::default() }
    }
}

pub enum EntityData {
    Player(LivingData),
    Skeleton(LivingData)
}

impl Entity {
    pub fn new(id: u32, pos: Vec2u, data: EntityData) -> Entity {
        return Entity { id, pos, data }
    }

    pub fn repr(&self) -> StyledContent<&str> {
        match &self.data {
            EntityData::Player(data) => match &data.direction {
                Direction::Up => "↑&",
                Direction::Down => "↓&",
                Direction::Left => "←&",
                Direction::Right => "→&"
            }.magenta().bold(),
            EntityData::Skeleton(_) => " k".bold()
        }
    }
}

pub trait EntityBehavior {
    fn think(&self, pos: &Vec2u, map: &Map) -> Action;
    fn update(&mut self, action: Action);
}

impl EntityBehavior for LivingData {
    fn think(&self, pos: &Vec2u, map: &Map) -> Action {
        self.action
    }

    fn update(&mut self, action: Action) {
        self.action = Action::Idle;
    }
}

impl LivingData {
    pub fn damage(&mut self, amount: u32) -> u32 {
        self.health = self.health.saturating_sub(amount);
        self.health
    }
}

impl EntityData {
    pub fn think(&self, pos: &Vec2u, map: &Map) -> Action {
        match self {
            EntityData::Player(living) => {
                // player-specific pre logic (if needed)
                living.think(pos, map)
            }
            EntityData::Skeleton(living) => {
                // skeleton-specific AI here
                living.think(pos, map)
            }
        }
    }

    pub fn update(&mut self, action: Action) {
        match self {
            EntityData::Player(living) => {
                // player-specific pre logic (if needed)
                living.update(action)
            }
            EntityData::Skeleton(living) => {
                // skeleton-specific AI here
                living.update(action)
            }
        }
    }
}
