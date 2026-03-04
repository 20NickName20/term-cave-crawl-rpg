use std::{collections::HashMap};
use crate::{util::vec2::{Vec2i, Vec2u}, world::{entity::{Action, Entity, EntityData}, generation::MapType, world::{TargetInfo, WorldAction}}};

use super::tile::Tile;

pub struct Map {
    width: u16,
    height: u16,
    level: i32,  
    gen_type: MapType,
    unlinked_warps: HashMap<Vec2u, i32>,
    tiles: Vec<Tile>,
    entities: HashMap<u32, Entity>,
    entity_id_counter: u32,
}

impl Map {
    pub fn new(width: u16, height: u16, value: &Tile, gen_type: MapType, level: i32) -> Self {
        let mut tiles = Vec::with_capacity((width * height) as usize);
        tiles.resize((width * height) as usize, value.clone());
        Map {
            width, height, tiles,
            entities: HashMap::new(),
            unlinked_warps: HashMap::new(),
            level,
            gen_type,
            entity_id_counter: 0
        }
    }

    pub fn get_width(&self) -> u16 { self.width }
    pub fn get_height(&self) -> u16 { self.height }
    pub fn get_type(&self) -> MapType { self.gen_type }
    pub fn get_level(&self) -> i32 { self.level }

    pub fn tile_at(&self, x: u16, y: u16) -> &Tile {
        &self.tiles[(y * self.width + x) as usize]
    }
    pub fn tile_at_mut(&mut self, x: u16, y: u16) -> &mut Tile {
        &mut self.tiles[(y * self.width + x) as usize]
    }
    pub fn is_passable(&self, x: u16, y: u16) -> bool {
        self.tiles[(y * self.width + x) as usize].is_passable()
    }

    pub fn fill<F>(&mut self, f: F) where F: Fn(u16, u16) -> Tile {
        for x in 0..self.width {
            for y in 0..self.height {
                let tile = f(x, y);
                if let Some(level_delta) = tile.get_level_delta() {
                    self.add_unlinked_warp(Vec2u::new(x, y), self.level + level_delta);
                    dbg!("Registered unlinked warp");
                }
                *self.tile_at_mut(x, y) = tile;
            }
        }
    }

    fn next_entity_id(&mut self) -> u32 {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        id
    }
    
    pub fn get_ids(&self) -> Vec<u32> {
        let count = self.entities.len();
        let mut keys: Vec<u32> = Vec::with_capacity(count);

        for key in self.entities.keys() {
            keys.push(*key);
        };

        keys
    }

    pub fn add_unlinked_warp(&mut self, pos: Vec2u, level: i32) {
        self.unlinked_warps.insert(pos, level);
    }

    pub fn link_warp(&mut self, target_level: i32, source_map_id: u32, source_pos: Vec2u) -> Result<Vec2u, String> {
        let mut found_pos: Option<Vec2u> = None;
        for (pos, _) in self.unlinked_warps.iter().filter(|(_, level)| {**level == target_level}) {
            found_pos = Some(pos.clone());
            break;
        }
        if let Some(pos) = found_pos {
            self.unlinked_warps.remove(&pos);
            let tile = self.tile_at_mut(pos.x, pos.y);
            tile.set_target_info(Some(TargetInfo {
                map_id: source_map_id,
                target_pos: source_pos
            }));
            Ok(pos)
        } else {
            Err("No free warps found".to_string())
        }
    }

    pub fn add_entity(&mut self, data: EntityData, pos: Vec2u) -> u32 {
        let id = self.next_entity_id();
        self.entities.insert(id, Entity::new(id, pos, data));
        return id;
    }

    pub fn get_entity(&self, id: u32) -> Result<&Entity, String> {
        if let Some(entity) = self.entities.get(&id) {
            return Ok(entity);
        }

        return Err("Could not find entity".to_string());
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Result<&mut Entity, String> {
        if let Some(entity) = self.entities.get_mut(&id) {
            return Ok(entity);
        }

        return Err("Could not find entity".to_string());
    }

    pub fn remove_entity(&mut self, id: u32) -> Result<EntityData, String> {
        if let Some(entity) = self.entities.remove(&id) {
            return Ok(entity.data);
        } else {
            return Err("Could not find entity".to_string());
        }
    }

    pub fn entities_by_pos(&self) -> HashMap<Vec2u, &Entity> {
        let mut map: HashMap<Vec2u, &Entity>  = HashMap::new();
        for entity in self.entities.values() {
            map.insert(entity.pos, entity);
        }
        return map;
    }

    pub fn update(&mut self) -> Result<Vec<WorldAction>, String> {
        let count = self.entities.len();
        let mut actions: Vec<(u32, Action)> = Vec::with_capacity(count);

        for (id, entity) in self.entities.iter() {
            actions.push((*id, entity.data.think(&entity.pos, self)));
        }
        
        Ok(self.resolve_actions(actions))
    }

    fn resolve_actions(&mut self, actions: Vec<(u32, Action)>) -> Vec<WorldAction> {
        let mut world_actions = Vec::new();
        for (id, action) in actions {
            match action {
                Action::Move(direction) => {
                    let entity: &Entity = match self.entities.get(&id) {
                        Some(e) => e,
                        None => continue,
                    };
                    let new_pos: Vec2i = entity.pos + direction.to_vec2();
                    if (0..self.width as i32).contains(&new_pos.x) && (0..self.height as i32).contains(&new_pos.y) {
                        if self.is_passable(new_pos.x as u16, new_pos.y as u16) {
                            if let Some(entity) = self.entities.get_mut(&id) {
                                entity.pos = Vec2u::try_from(new_pos).unwrap();
                            }
                        }
                    }
                },
                Action::UseWarpTile => {
                    let entity: &Entity = match self.entities.get(&id) {
                        Some(e) => e,
                        None => continue,
                    };
                    if let Some(_) = self.tile_at(entity.pos.x, entity.pos.y).get_level_delta() {
                        world_actions.push(WorldAction::EntityUseWarp { entity_id: id });
                    }
                }
                _ => ()
            }
            self.entities.get_mut(&id).unwrap().data.update(action);
        }

        world_actions
    }
}
