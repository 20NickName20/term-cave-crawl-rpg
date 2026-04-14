use std::collections::HashMap;

use crate::world::generation::world_gen::Level;
use crate::world::{entity::Entity, map::Map};
use crate::util::vec2::Vec2u;

#[derive(Clone)]
pub struct TargetInfo {
    pub map_id: u32,
    pub pos: Vec2u
}

pub enum WorldAction {
    EntityUseWarp {
        entity_id: u32
    }
}

#[derive(Debug)]
pub enum WorldEvent {
    EntiyMovedToMap {
        old_entity_id: u32,
        new_entity_id: u32,
        target_map_id: u32
    }
}

pub struct World {
    maps: HashMap<u32, Map>,
    map_id_counter: u32,
    level_graphs: Vec<Level>
}

impl World {
    pub fn new() -> Self {
        World {
            maps: HashMap::new(),
            map_id_counter: 0,
            level_graphs: Vec::new()
        }
    }

    pub fn get_next_level_idx(&self) -> u32 {
        self.level_graphs.len() as u32
    }

    pub fn add_level(&mut self, level: Level) {
        self.level_graphs.push(level);
    }

    pub fn get_last_level(&self) -> Option<&Level> {
        self.level_graphs.last()
    }

    fn next_map_id(&mut self) -> u32 {
        let id = self.map_id_counter;
        self.map_id_counter += 1;
        id
    }

    pub fn get_next_map_id(&self) -> u32 {
        self.map_id_counter
    }

    pub fn add_map(&mut self, map: Map) -> u32 {
        let id = self.next_map_id();
        self.maps.insert(id, map);
        id
    }

    pub fn get_map(&self, id: u32) -> Option<&Map> {
        self.maps.get(&id)
    }

    fn get_map_mut(&mut self, id: u32) -> Option<&mut Map> {
        self.maps.get_mut(&id)
    }

    pub fn get_entity_on_map_mut(&mut self, map_id: u32, entity_id: u32) -> Option<&mut Entity> {
        self.get_map_mut(map_id)?.get_entity_mut(entity_id)
    }

    pub fn update_map(&mut self, map_id: u32) -> Result<Vec<WorldEvent>, String> {
        if let Some(map) = self.maps.get_mut(&map_id) {
            let mut events = Vec::new();
            for world_action in map.update()? {
                match world_action {
                    WorldAction::EntityUseWarp { entity_id } => {
                        let info = self.entity_use_warp(map_id, entity_id)?;
                        events.push(WorldEvent::EntiyMovedToMap {
                            old_entity_id: entity_id,
                            target_map_id: info.0,
                            new_entity_id: info.1
                        });
                    }
                };
            }
            Ok(events)
        } else {
            Err("Map not found".to_string())
        }
    }

    pub fn update_all(&mut self) -> Result<Vec<WorldEvent>, String> {
        let mut ids = Vec::with_capacity(self.maps.len());
        let mut events: Vec<WorldEvent> = vec![];
        self.maps.keys().for_each(|i| {ids.push(*i);});
        for id in ids {
            events.append(&mut self.update_map(id)?);
        }
        Ok(events)
    }

    fn entity_use_warp(&mut self, map_id: u32, entity_id: u32) -> Result<(u32, u32), String> {
        let map = self.get_map(map_id).ok_or("No map info".to_string())?;
        let entity = map.get_entity(entity_id).ok_or("No entity".to_string())?;
        let target_info = map.warp_at(entity.pos.x, entity.pos.y).ok_or("No target info".to_string())?;

        let target_id = target_info.map_id;
        let new_entity_id = self.transfer_entity(map_id, entity_id, target_info.clone())?;
        Ok((target_id, new_entity_id))
    }

    fn transfer_entity(&mut self, source_map_id: u32, entity_id: u32, target: TargetInfo) -> Result<u32, String> {
        let source_map = self.maps.get_mut(&source_map_id).ok_or("Source map not found".to_string())?;
        let entity = source_map.remove_entity(entity_id).ok_or("Entity not found".to_string())?;
        let target_map = self.maps.get_mut(&target.map_id).ok_or("Target map not found".to_string())?;
        let id = target_map.add_entity(entity, target.pos);
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::{direction::Direction, vec2::Vec2u}, world::{entity::{EntityData, LivingData}, generation::map_gen, world::{TargetInfo, World}}};

    #[test]
    fn entity_transfer() -> Result<(), String> {
        let mut map = map_gen::generate_map(map_gen::MapType::Test, 0);
        let mut player_id = map.add_entity(EntityData::Player(LivingData::new(100, Direction::Right)), Vec2u::new(3, 3));
        assert_eq!(map.get_entity(player_id).expect("Expected entity").pos, Vec2u::new(3, 3));
        assert_eq!(player_id, 0u32);
        let mut world = World::new();
        let map_id = world.add_map(map);
        assert_eq!(map_id, 0u32);

        let map2 = map_gen::generate_map(map_gen::MapType::Test, 0);
        let map2_id = world.add_map(map2);
        assert_eq!(map2_id, 1u32);

        player_id = world.transfer_entity(map_id, player_id, TargetInfo {
            map_id: map2_id,
            pos: Vec2u::new(6, 7)
        })?;

        assert_eq!(player_id, 0);
        assert_eq!(world.get_map(map2_id).expect("Expected map").get_entity(player_id).expect("Expected entity").pos, Vec2u::new(6, 7));

        player_id = world.transfer_entity(map2_id, player_id, TargetInfo {
            map_id: map_id,
            pos: Vec2u::new(1, 1)
        })?;
        assert_eq!(player_id, 1);
        assert_eq!(
            world.get_map(map_id).expect("Expected map")
                .get_entity(player_id).expect("Expected entity").pos,
            Vec2u::new(1, 1)
        );

        Ok(())
    }
}
