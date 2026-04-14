use rand::RngExt;

use crate::{log, util::vec2::Vec2u, world::{map::Map, tile::Tile}};

#[derive(Clone, Copy, Default)]
pub enum MapType {
    #[default]
    Test,
    Initial,
    RockCave,
    FloodedRockCave,
    RockCaveWithLakes,
    RockCaveWithLava,
    Labyrinth
}

impl MapType {
    pub fn default_tile(&self) -> Tile {
        match self {
            MapType::Test => Tile::Floor,
            MapType::Initial => Tile::Floor,
            MapType::Labyrinth => Tile::Floor,
            MapType::RockCave => Tile::RockFloor,
            MapType::FloodedRockCave => Tile::RockFloor,
            MapType::RockCaveWithLakes => Tile::RockFloor,
            MapType::RockCaveWithLava => Tile::RockFloor
        }
    }
}

fn get_size_for(map_type: MapType, level: i32) -> (u16, u16) {
    match map_type {
        MapType::Test => (15 + (level.unsigned_abs() as u16) * 6, 15 + (level.unsigned_abs() as u16) * 6),
        MapType::Initial => {
            let w: u16 = rand::random_range(30..51);
            let h: u16 = rand::random_range(30..51);
            (w, h)
        },
        _ => {
            let w: u16 = rand::random_range(10..100);
            let h: u16 = rand::random_range(10..100);
            (w, h)
        }
    }
}

pub fn generate_map(gen_type: MapType, level: i32) -> Map {
    let (width, height) = get_size_for(gen_type, level);
    let mut map = Map::new(width, height, &gen_type.default_tile());
    
    match gen_type {
        MapType::Initial => generate_initial(&mut map),
        MapType::Test => generate_test(&mut map),
        _ => ()
    }

    map
}

fn generate_initial(map: &mut Map) {
    let (w, h) = map.get_size();
    let mut rng = rand::rng();

    let tile_count = (w * h) as usize;
    let trees: Vec<bool> = {
        let mut buf1 = vec![false; tile_count];
        
        for x in 0..w {
            for y in 0..h {
                let idx = (y * w + x) as usize;
                if x < 2 || x > w - 3 || y < 2 || y > h - 3 {
                    buf1[idx] = true;
                } else {
                    buf1[idx] = rng.random_bool(1.0 / 3.0);
                }
            }
        }

        let mut buf2 = vec![true; tile_count];
        for x in 1..(w-1) {
            for y in 1..(h-1) {
                let mut count: u16 = 0;
                for dx in -1..=1i32 {
                    for dy in -1..=1i32 {
                        let x = x as i32 + dx;
                        let y = y as i32 + dy;
                        if buf1[(y * (w as i32) + x) as usize] {
                            count += 1;
                        }
                    }
                }
                buf2[(y * w + x) as usize] = count > 4;
            }
        }

        buf2
    };

    let entrance_x = rng.random_range(4..(w-4));
    let entrance_y = rng.random_range(4..(h-4));

    let x_scale = rng.random_range(60..141);
    let y_scale = rng.random_range(60..141);
    
    map.fill(|x, y| {
        if x == entrance_x && y == entrance_y {
            return Tile::StairDown;
        }
        let dx = entrance_x.abs_diff(x) * x_scale / 100;
        let dy = entrance_y.abs_diff(y) * y_scale / 100;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < 25 {
            return Tile::RockFloor;
        }
        if trees[(y * w + x) as usize] {
            return Tile::Tree;
        }
        Tile::Grass
    });
}

fn generate_test(map: &mut Map) {
    let rand_val = rand::random_range(0..3);
    map.fill(|x, y| {
        if x.rem_euclid(18) == 4 && y.rem_euclid(18) == 4 {
            let _ = log!("Placing unlinked warp tile");
            return match ((x + y) / 18 + rand_val) % 3 {
                0 => Tile::StairDown,
                1 => Tile::StairUp,
                _ => Tile::Tunnel
            }
        }
        if x.rem_euclid(6) < 3 && y.rem_euclid(6) < 3 {
            Tile::Wall
        } else {
            Tile::Floor
        }
    });
}
