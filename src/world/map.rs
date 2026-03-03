use std::{collections::HashMap};
use crate::{util::vec2::Vec2, world::{entity::{Action, Entity, EntityData}, generation::MapType}};

use super::tile::Tile;

pub struct Map {
    width: u16,
    height: u16,
    level: i32,  
    gen_type: MapType,
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
            level,
            gen_type,
            entity_id_counter: 1
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

    pub fn fill<F>(&mut self, f: F) where F: Fn(u16) -> Tile {
        for i in 0..self.tiles.len() {
            self.tiles[i] = f(i as u16);
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

    pub fn add_entity(&mut self, data: EntityData, pos: Vec2) -> u32 {
        let id = self.next_entity_id();
        self.entities.insert(id, Entity::new(id, pos, data));
        return id;
    }

    pub fn get_entity(&self, id: &u32) -> Result<&Entity, String> {
        if let Some(entity) = self.entities.get(id) {
            return Ok(entity);
        }

        return Err("Could not find entity".to_string());
    }

    pub fn get_entity_mut(&mut self, id: &u32) -> Result<&mut Entity, String> {
        if let Some(entity) = self.entities.get_mut(id) {
            return Ok(entity);
        }

        return Err("Could not find entity".to_string());
    }

    pub fn remove_entity(&mut self, id: &u32) -> Result<EntityData, String> {
        if let Some(entity) = self.entities.remove(id) {
            return Ok(entity.data);
        } else {
            return Err("Could not find entity".to_string());
        }
    }

    pub fn entities_by_pos(&self) -> HashMap<Vec2, &Entity> {
        let mut map: HashMap<Vec2, &Entity>  = HashMap::new();
        for entity in self.entities.values() {
            map.insert(entity.pos, entity);
        }
        return map;
    }

    pub fn update_all(&mut self) -> Result<(), String> {
        let count = self.entities.len();
        let mut actions: Vec<(u32, Action)> = Vec::with_capacity(count);

        for (id, entity) in self.entities.iter() {
            actions.push((*id, entity.data.think(&entity.pos, self)));
        }

        self.resolve_actions(actions);
        
        Ok(())
    }

    fn resolve_actions(&mut self, actions: Vec<(u32, Action)>) {
        for (id, action) in actions {
            match action {
                Action::Move(direction) => {
                    let entity = match self.entities.get(&id) {
                        Some(e) => e,
                        None => continue,
                    };
                    let new_pos = entity.pos + direction.to_vec2();
                    if (0..self.width).contains(&new_pos.x_u16()) && (0..self.height).contains(&new_pos.y_u16()) {
                        if self.is_passable(new_pos.x_u16(), new_pos.y_u16()) {
                            if let Some(entity) = self.entities.get_mut(&id) {
                                entity.pos = new_pos;
                            }
                        }
                        self.entities.get_mut(&id).unwrap().data.update(action);
                    }
                },
                _ => ()
            }
        }
    }
}
