use crate::{util::{direction, vec2::Vec2u}, world::{map::Map, tile::Tile}};

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
        MapType::Test => (50, 50),
        _ => {
            let w: u16 = rand::random_range(10..100);
            let h: u16 = rand::random_range(10..100);
            (w, h)
        }
    }
}

pub fn generate_map(gen_type: MapType, level: i32, direction: Option<i32>) -> Map {
    let (width, height) = get_size_for(gen_type, level);
    let map = Map::new(width, height, &gen_type.default_tile(), gen_type, level);
    
    generate(map, gen_type, level, direction)
}

fn generate(map: Map, gen_type: MapType, level: i32, direction: Option<i32>) -> Map {
    match gen_type {
        MapType::Test => generate_test(map, direction),
        _ => map
    }
}

fn place_warps_basic(mut map: Map, count: u16) -> Map {
    let (w, h) = (map.get_width(), map.get_height());

    let mut placed = 0;
    while placed < count {
        let x: u16 = rand::random_range(0..w);
        let y: u16 = rand::random_range(0..h);
        if !map.is_passable(x, y) {continue;}
        if map.tile_at(x, y).get_level_delta().is_some() {continue;}
        let t: u8 = rand::random_range(0..6);
        let (tile, level_delta) = match t {
            0 => (Tile::StairUp(None), -1),
            1 | 2 => (Tile::Tunnel(None), 0),
            _ => (Tile::StairDown(None), 1)
        };
        *map.tile_at_mut(x, y) = tile;
        map.add_unlinked_warp(Vec2u::new(x, y), level_delta);
        placed += 1;
    }

    map
}

fn generate_test(mut map: Map, direction: Option<i32>) -> Map {
    let rand_val = rand::random_range(0..3);
    map.fill(|x, y| {
        if let Some(direction) = direction {
            if x == 10 && y == 10 {
                return Tile::warp_from_direction(-direction, None);
            }
        }
        if x.rem_euclid(18) == 4 && y.rem_euclid(18) == 4 {
            dbg!("Placing warp tile to unknown location");
            return match ((x + y) / 18 + rand_val) % 3 {
                0 => Tile::StairDown(None),
                1 => Tile::StairUp(None),
                _ => Tile::Tunnel(None)
            }
        }
        if x.rem_euclid(6) < 3 && y.rem_euclid(6) < 3 {
            Tile::Wall
        } else {
            Tile::Floor
        }
    });

    map
}
