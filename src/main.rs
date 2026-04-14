mod util;
mod app;
mod render;
mod world;

use crossterm::event::{Event, KeyCode, KeyEvent};

use app::App;

use crate::{util::{direction::Direction, vec2::Vec2u}, world::{entity::{Action, EntityData, LivingData}, generation::map_gen::{self, MapType}, map::Map, world::{World, WorldEvent}}};

struct GameData {
    world: World,
    current_map_id: u32,
    player_id: u32,
    should_clear_screen: bool
}

impl GameData {
    pub fn current_map(&self) -> &Map {
        self.world.get_map(self.current_map_id).unwrap()
    }

    pub fn camera_pos(&self) -> Vec2u {
        if let Some(player) = &self.current_map().get_entity(self.player_id) {
            player.pos
        } else {
            Vec2u::new(0, 0)
        }
    }
}

fn main_loop(app: &mut App<GameData>) -> Result<(), String> {
    for action in app.data.world.update_all()? {
        match action {
            WorldEvent::EntiyMovedToMap { old_entity_id, new_entity_id, target_map_id } => {
                if app.data.player_id == old_entity_id {
                    app.data.player_id = new_entity_id;
                    app.data.current_map_id = target_map_id;
                    app.data.should_clear_screen = true;
                }
            }
        }
    }

    if let Err(msg) = render::render(app) {
        return Err(msg.to_string());
    }

    Ok(())
}

fn handle_key(app: &mut App<GameData>, event: KeyEvent) -> Result<(), String> {
    if !event.is_press() {
        return Ok(())
    }

    if let EntityData::Player(living) = &mut app.data.world.get_entity_on_map_mut(app.data.current_map_id, app.data.player_id).unwrap().data {
        let player_action: Action = match event.code {
            KeyCode::Left => Action::Move(Direction::Left),
            KeyCode::Right => Action::Move(Direction::Right),
            KeyCode::Up => Action::Move(Direction::Up),
            KeyCode::Down => Action::Move(Direction::Down),
            KeyCode::Char('>') => Action::UseWarpTile,
            _ => Action::Idle
        };
        living.action = player_action;
        if let Action::Move(direction) = player_action {
            living.direction = direction;
        }
    }

    Ok(())
}

fn handle_event(app: &mut App<GameData>, event: Event) -> Result<(), String> {
    match event {
        Event::Key(key) => handle_key(app, key),
        Event::Resize(_, _) => {
            app.data.should_clear_screen = true;
            Ok(())
        },
        _ => Ok(())
    }
}

fn main() -> Result<(), String> {
    log!("[]==--------------==[]")?;
    log!("[] Program started. []")?;
    log!("[]==--------------==[]")?;
    let mut map = map_gen::generate_map(MapType::Initial, 0);
    let spawn_pos = Vec2u::new(map.get_width() / 2, map.get_height() / 2);
    let player_id = map.add_entity(EntityData::Player(LivingData::new(100, Direction::Right)), spawn_pos);
    let mut world = World::new();
    let map_id = world.add_map(map);

    let data = GameData {
        world,
        current_map_id: map_id,
        player_id,
        should_clear_screen: false
    };
    let mut app = App::new(data);

    app.main(main_loop, handle_event)
}
