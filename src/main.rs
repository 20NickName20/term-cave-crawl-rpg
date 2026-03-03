mod util;
mod app;
mod render;
mod world;

use crossterm::event::Event;

use util::vec2::Vec2;
use app::App;

use crate::{util::direction::Direction, world::{entity::{Action, EntityData, LivingData}, generation, map::Map}};

struct GameData {
    map: Map,
    player_id: u32
}

impl GameData {
    pub fn current_map(&self) -> &Map {
        &self.map
    }

    pub fn camera_pos(&self) -> &Vec2 {
        &self.map.get_entity(&self.player_id).unwrap().pos
    }
}

fn main_loop(app: &mut App<GameData>) -> Result<(), String> {
    if let Err(msg) = render::render(app) {
        return Err(msg.to_string());
    }

    app.data.map.update_all()?;
    
    Ok(())
}

fn handle_event(app: &mut App<GameData>, event: Event) -> Result<(), String> {
    let Event::Key(key) = event else {
        return Ok(());
    };

    if !key.is_press() {
        return Ok(())
    }

    if let EntityData::Player(living) = &mut app.data.map.get_entity_mut(&app.data.player_id).unwrap().data {
        let player_move: Option<Direction> = match key.code {
            crossterm::event::KeyCode::Left => Some(Direction::Left),
            crossterm::event::KeyCode::Right => Some(Direction::Right),
            crossterm::event::KeyCode::Up => Some(Direction::Up),
            crossterm::event::KeyCode::Down => Some(Direction::Down),
            _ => None
        };
        if let Some(direction) = player_move {
            living.action = Action::Move(direction);
            living.direction = direction;
        } 
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut map = generation::generate_map(generation::MapType::Test, 0);
    let player_id = map.add_entity(EntityData::Player(LivingData::new(100, Direction::Right)), Vec2::new(3, 3));

    let data = GameData {map, player_id};
    let mut app = App::new(data);

    app.main(main_loop, handle_event)
}
