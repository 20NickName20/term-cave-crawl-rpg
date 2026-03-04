mod util;
mod app;
mod render;
mod world;

use crossterm::event::{Event, KeyCode};

use util::vec2::Vec2;
use app::App;

use crate::{util::{direction::Direction, vec2::Vec2u}, world::{entity::{Action, EntityData, LivingData}, generation, map::Map, world::{World, WorldEvent}}};

struct GameData {
    world: World,
    current_map_id: u32,
    player_id: u32
}

impl GameData {
    pub fn current_map(&self) -> &Map {
        &self.world.get_map(self.current_map_id).unwrap()
    }

    pub fn camera_pos(&self) -> Vec2u {
        if let Ok(player) = &self.current_map().get_entity(self.player_id) {
            player.pos
        } else {
            Vec2u::new(0, 0)
        }
    }
}

fn main_loop(app: &mut App<GameData>) -> Result<(), String> {
    if let Err(msg) = render::render(app) {
        return Err(msg.to_string());
    }

    for action in app.data.world.update_all()? {
        match action {
            WorldEvent::EntiyMovedToMap { old_entity_id, new_entity_id, target_map_id } => {
                if app.data.player_id == old_entity_id {
                    app.data.player_id = new_entity_id;
                    app.data.current_map_id = target_map_id;
                }
            }
        }
    }
    
    Ok(())
}

fn handle_event(app: &mut App<GameData>, event: Event) -> Result<(), String> {
    let Event::Key(key) = event else {
        return Ok(());
    };

    if !key.is_press() {
        return Ok(())
    }

    if let EntityData::Player(living) = &mut app.data.world.get_entity_on_map_mut(app.data.current_map_id, app.data.player_id).unwrap().data {
        let player_action: Action = match key.code {
            KeyCode::Left => Action::Move(Direction::Left),
            KeyCode::Right => Action::Move(Direction::Right),
            KeyCode::Up => Action::Move(Direction::Up),
            KeyCode::Down => Action::Move(Direction::Down),
            KeyCode::Char(c) => match c {
                '>' => Action::UseWarpTile,
                _ => Action::Idle
            },
            _ => Action::Idle
        };
        living.action = player_action;
        if let Action::Move(direction) = player_action {
            living.direction = direction;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut map = generation::generate_map(generation::MapType::Test, 0, None);
    let player_id = map.add_entity(EntityData::Player(LivingData::new(100, Direction::Right)), Vec2::new(3, 3));
    let mut world = World::new();
    let map_id = world.add_map(map);

    let data = GameData {
        world,
        current_map_id: map_id,
        player_id
    };
    let mut app = App::new(data);

    app.main(main_loop, handle_event)
}
#[cfg(test)]
mod tests {
    use crate::world::world::TargetInfo;

    use super::*;

    #[test]
    fn entity_transfer() -> Result<(), String> {
        let mut map = generation::generate_map(generation::MapType::Test, 0, None);
        let mut player_id = map.add_entity(EntityData::Player(LivingData::new(100, Direction::Right)), Vec2u::new(3, 3));
        assert_eq!(map.get_entity(player_id)?.pos, Vec2u::new(3, 3));
        assert_eq!(player_id, 0u32);
        let mut world = World::new();
        let map_id = world.add_map(map);
        assert_eq!(map_id, 0u32);

        let map2 = generation::generate_map(generation::MapType::Test, 0, None);
        let map2_id = world.add_map(map2);
        assert_eq!(map2_id, 1u32);

        player_id = world.transfer_entity(map_id, player_id, TargetInfo {
            map_id: map2_id,
            target_pos: Vec2u::new(6, 7)
        })?;

        assert_eq!(player_id, 0);
        assert_eq!(world.get_map(map2_id)?.get_entity(player_id)?.pos, Vec2u::new(6, 7));

        player_id = world.transfer_entity(map2_id, player_id, TargetInfo {
            map_id: map_id,
            target_pos: Vec2u::new(1, 1)
        })?;
        assert_eq!(player_id, 1);
        assert_eq!(world.get_map(map_id)?.get_entity(player_id)?.pos, Vec2u::new(1, 1));

        Ok(())
    }

    #[test]
    fn map_linking() -> Result<(), String> {
        todo!()
    }
}