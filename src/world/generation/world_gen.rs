use std::{collections::HashSet, u32};

use rand::{RngExt, seq::{IndexedMutRandom, IndexedRandom}};

use crate::world::world::{TargetInfo, World};

enum LevelType {
    Cave
}

pub struct Level {
    exits: Vec<TargetInfo>
}

fn sort_pair(p: (u32, u32)) -> (u32, u32) {
    if p.0 < p.1 {
        p
    } else {
        (p.1, p.0)
    }
}

fn get_maps_per_level(level: u32, entrances: u32) -> u32 {
    3 + level
}

pub fn generate_next_level(world: &mut World) {
    let level_idx = world.get_next_level_idx();
    let prev_level = world.get_last_level().expect("No last level");
    let entrance_count = prev_level.exits.len() as u32;
    let rng = &mut rand::rng();
    let map_count = get_maps_per_level(level_idx, entrance_count);
    let cluster_count = rng.random_range(1..=entrance_count);
    let mut clusters: Vec<Vec<u32>> = Vec::with_capacity(cluster_count as usize);
    for _ in 0..cluster_count {
        clusters.push(vec![]);
    }
    for i in 0..map_count {
        clusters.choose_mut(rng).expect("Unexpected empty vector").push(
            world.get_next_map_id() + i
        );
    }

    let mut non_empty_clusters = Vec::new();
    for cluster in clusters {
        if !cluster.is_empty() {
            non_empty_clusters.push(cluster);
        }
    }
    let clusters = non_empty_clusters;
    let mut connections = HashSet::new();
    for cluster in clusters {
        let count = cluster.len() - 1;
        if count == 0 {
            continue;
        }
        for i in 0..count {
            connections.insert(
                sort_pair((
                    cluster[i],
                    cluster[i+1]
                ))
            );
        }

        for _ in 0..rng.random_range(0..cluster.len() / 2) {
            let u = *cluster.choose(rng).unwrap();
            let v = *cluster.choose(rng).unwrap();
            if u != v {
                connections.insert(
                    sort_pair((u, v))
                );
            }
        }
    }

    world.add_level(Level { exits: todo!() });
}

