use std::io::{self, Stdout, Write};

use crossterm::style::{Print, ResetColor};
use crossterm::{cursor, queue, terminal};

use crate::GameData;
use crate::app::App;
use crate::util::vec2::Vec2;
use crate::world::map::Map;

fn draw_map(stdout: &mut Stdout, map: &Map, cam_pos: &Vec2, render_width: u16) -> io::Result<()> {
    let size = terminal::size()?;
    let cam_width = size.0.min(render_width) / 2 - 1;
    let cam_height = size.1 - 1;
    let map_width = map.get_width();
    let map_height = map.get_height();
    let cam_x = cam_pos.x_u16();
    let cam_y = cam_pos.y_u16();

    let entities = map.entities_by_pos();
    
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        ResetColor
    )?;
    
    let half_w = cam_width / 2;
    let half_h = cam_height / 2;
    let start_x = (cam_x.saturating_sub(half_w)).min(map_width.saturating_sub(cam_width));
    let start_y = (cam_y.saturating_sub(half_h)).min(map_height.saturating_sub(cam_height));
    let end_x = map_width.min(start_x + cam_width);
    let end_y = map_height.min(start_y + cam_height);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let repr = if let Some(entity) = entities.get(&Vec2::new(x.into(), y.into())) {
                entity.repr()
            } else {
                map.tile_at(x, y).repr()
            };

            queue!(
                stdout,
                Print(repr)
            )?;
        }
        queue!(
            stdout,
            Print("\n\r"),
        )?;
    }

    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )?;

    stdout.flush()
}

pub fn render(app: &mut App<GameData>) -> io::Result<()> {
    draw_map(
        &mut app.stdout,
        app.data.current_map(),
        app.data.camera_pos(),
        terminal::size().unwrap().0
    )?;

    Ok(())
}