use crate::world::{map::Map, tile::{self, Tile}};

#[derive(Clone, Copy, Default)]
pub enum MapType {
    #[default]
    Test,
    RockCave,
    FloodedRockCave,
    RockCaveWithLakes,
    RockCaveWithLava
}

impl MapType {
    fn default_tile(&self) -> Tile {
        match self {
            MapType::Test => Tile::Floor,
            MapType::RockCave => Tile::RockFloor,
            MapType::FloodedRockCave => Tile::RockFloor,
            MapType::RockCaveWithLakes => Tile::RockFloor,
            MapType::RockCaveWithLava => Tile::RockFloor
        }
    }
}

fn get_size_for(map_type: MapType, level: i32) -> (u16, u16) {
    match map_type {
        MapType::Test => (255, 255),
        _ => {
            let w: u16 = rand::random_range(10..100);
            let h: u16 = rand::random_range(10..100);
            (w, h)
        }
    }
}

pub fn generate_map(gen_type: MapType, level: i32) -> Map {
    let (width, height) = get_size_for(gen_type, level);
    let mut map = Map::new(width, height, &gen_type.default_tile(), gen_type, level);

    generate(map, gen_type, level)
}

fn generate(mut map: Map, gen_type: MapType, level: i32) -> Map {
    match gen_type {
        MapType::Test => generate_test(map),
        _ => map
    }
}

fn generate_test(mut map: Map) -> Map {
    map.fill(|_| {
        if rand::random::<f32>() < 0.1 {
            Tile::Wall
        } else {
            Tile::Floor
        }
    });

    map
}
