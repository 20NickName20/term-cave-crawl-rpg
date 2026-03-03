use std::collections::HashMap;

use crate::{util::vec2::Vec2, world::map::Map};

#[derive(Clone)]
pub struct TargetInfo {
    map_id: u32,
    target_pos: Vec2
}

pub struct World {
    maps: HashMap<u32, Map>,
    next_map_id: u32
}

impl World {
    pub fn move_entity(&mut self, source_map_id: u32, entity_id: u32, target: TargetInfo) -> Result<u32, String> {
        let Some(source_map) = self.maps.get_mut(&source_map_id) else {
            return Err("Source map not found".to_string())
        };
        let entity = source_map.remove_entity(&entity_id)?;
        let Some(target_map) = self.maps.get_mut(&target.map_id) else {
            return Err("Target map not found".to_string())
        };
        let id = target_map.add_entity(entity, target.target_pos);
        Ok(id)
    }
}