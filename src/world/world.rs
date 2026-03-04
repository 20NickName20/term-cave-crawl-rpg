use std::collections::HashMap;

use crate::world::{entity::Entity, generation, map::Map};
use crate::util::vec2::Vec2u;

#[derive(Clone)]
pub struct TargetInfo {
    pub map_id: u32,
    pub target_pos: Vec2u
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
    map_ids_by_level: HashMap<i32, Vec<u32>>,
    map_id_counter: u32
}

impl World {
    pub fn new() -> Self {
        World {
            maps: HashMap::new(),
            map_ids_by_level: HashMap::new(),
            map_id_counter: 0
        }
    }

    fn next_map_id(&mut self) -> u32 {
        let id = self.map_id_counter;
        self.map_id_counter += 1;
        id
    }

    pub fn add_map(&mut self, map: Map) -> u32 {
        let id = self.next_map_id();
        let level = map.get_level();
        self.maps.insert(id, map);
        self.map_ids_by_level.entry(level).or_default().push(id);
        id
    }

    pub fn get_map(&self, id: u32) -> Result<&Map, String> {
        if let Some(map) = self.maps.get(&id) {
            return Ok(map);
        } else {
            return Err("Could not find map".to_string());
        }
    }

    fn get_map_mut(&mut self, id: u32) -> Result<&mut Map, String> {
        if let Some(map) = self.maps.get_mut(&id) {
            return Ok(map);
        } else {
            return Err("Could not find map".to_string());
        }
    }

    pub fn get_entity_on_map_mut(&mut self, map_id: u32, entity_id: u32) -> Result<&mut Entity, String> {
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
                            new_entity_id: info.1,
                            target_map_id: info.0
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

    pub fn entity_use_warp(&mut self, map_id: u32, entity_id: u32) -> Result<(u32, u32), String> {
        let (current_level, depth_delta, pos, opt_target_info) = {
            let current_map = self.get_map_mut(map_id)?;
            let entity = current_map.get_entity(entity_id)?;
            let pos = entity.pos;

            let level = current_map.get_level();
            let tile = current_map.tile_at_mut(pos.x, pos.y);
            let depth_delta = tile
                .get_level_delta()
                .ok_or_else(|| "Tile is not a warp tile".to_string())?;

            let opt_target_info = tile.get_target_info().expect("Expected to be a warp tile");

            (level, depth_delta, pos, opt_target_info)
        };

        // either use the cloned info or create a new map and write it back
        let target_info = if let Some(info) = opt_target_info {
            info
        } else {
            let mut new_map = generation::generate_map(generation::MapType::Test, current_level + depth_delta, Some(depth_delta));
            let target_pos = new_map.link_warp(current_level, map_id, pos).expect("Expected new map to have a valid warp back");
            let target_id = self.add_map(new_map);
            dbg!("Generated new map", target_id);
            let info = TargetInfo { map_id: target_id, target_pos };

            // update the original tile now that the borrow has ended
            let current_map = self.get_map_mut(map_id)?;
            let tile = current_map.tile_at_mut(pos.x, pos.y);
            tile.set_target_info(Some(info.clone()));
            info
        };

        let target_id = target_info.map_id;
        let new_entity_id = self.transfer_entity(map_id, entity_id, target_info)?;
        Ok((target_id, new_entity_id))
    }

    pub fn transfer_entity(&mut self, source_map_id: u32, entity_id: u32, target: TargetInfo) -> Result<u32, String> {
        let Some(source_map) = self.maps.get_mut(&source_map_id) else {
            return Err("Source map not found".to_string())
        };
        let entity = source_map.remove_entity(entity_id)?;
        let Some(target_map) = self.maps.get_mut(&target.map_id) else {
            return Err("Target map not found".to_string())
        };
        let id = target_map.add_entity(entity, target.target_pos);
        Ok(id)
    }
}