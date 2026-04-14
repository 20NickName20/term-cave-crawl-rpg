use std::collections::{HashMap, HashSet};

use crate::{util::vec2::{Vec2, Vec2i, Vec2u}, world::{entity::{Action, Entity, EntityData}, world::{TargetInfo, WorldAction}}};

use super::tile::Tile;

pub struct Map {
    width: u16,
    height: u16,
    warps: HashMap<Vec2u, TargetInfo>,
    free_warps_to_next: HashSet<Vec2u>,
    tiles: Vec<Tile>,
    entities: HashMap<u32, Entity>,
    entity_id_counter: u32
}

impl Map {
    pub fn new(width: u16, height: u16, value: &Tile) -> Self {
        let tiles = vec![value.clone(); (width * height) as usize];
        Map {
            width, height, tiles,
            entities: HashMap::new(),
            warps: HashMap::new(),
            free_warps_to_next: HashSet::new(),
            entity_id_counter: 0
        }
    }

    pub fn get_width(&self) -> u16 { self.width }
    pub fn get_height(&self) -> u16 { self.height }
    pub fn get_size(&self) -> (u16, u16) { (self.width, self.height) }

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
                *self.tile_at_mut(x, y) = f(x, y);
            }
        }
    }

    pub fn warp_at(&self, x: u16, y: u16) -> Option<&TargetInfo> {
        self.warps.get(&Vec2u::new(x, y))
    }

    pub fn add_warp(&mut self, x: u16, y: u16, target_info: TargetInfo) {
        self.warps.insert(Vec2u::new(x, y), target_info);
    }

    pub fn add_free_warp(&mut self, x: u16, y: u16) {
        self.free_warps_to_next.insert(Vec2::new(x, y));
    }

    pub fn link_free_warp_to_next(&mut self, x: u16, y: u16, target_info: TargetInfo) {
        let pos = Vec2::new(x, y);
        self.free_warps_to_next.remove(&pos);
        self.warps.insert(pos, target_info);
    }

    fn next_entity_id(&mut self) -> u32 {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        id
    }

    pub fn add_entity(&mut self, data: EntityData, pos: Vec2u) -> u32 {
        let id = self.next_entity_id();
        self.entities.insert(id, Entity::new(id, pos, data));
        return id;
    }

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: u32) -> Option<EntityData> {
        self.entities.remove(&id).map(|e| e.data)
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
                    
                    if self.warps.contains_key(&entity.pos) {
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
