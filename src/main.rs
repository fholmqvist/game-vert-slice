use ::rand::thread_rng;
use constants::TILE_SIZE;
use entities::Position;
use hecs::World;
use level::Level;
use macroquad::prelude::*;
use utils::v2_to_index;

mod camera;
mod constants;
mod entities;
mod level;
mod resources;
mod spawn;
mod steps;
mod tile;
mod tiles;
mod utils;

#[macroquad::main("WERFS")]
async fn main() {
    let (werfs_texture, tileset_texture) = resources::load().await;

    let mut rng = thread_rng();

    let mut world = World::new();

    let level_path = "../resources/level_debug";
    let Ok(mut level) = Level::new(&mut rng, level_path) else {
        panic!("failed to load {}", level_path);
    };

    let mut cam = camera::Camera::new(Vec2 {
        x: screen_width() / 5.0,
        y: screen_height() / 5.0,
    });

    let mut total_werfs = 0;

    let first_entity = spawn::two_werfs(&mut total_werfs, &mut world, &mut rng);

    let mut elapsed = get_time();

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        let mut positions: Vec<Position> = Vec::with_capacity(total_werfs as usize);

        cam.set_cam(None);

        level.draw(&tileset_texture);

        steps::move_and_draw(&mut world, &werfs_texture, &mut positions);

        steps::collision(&mut world, positions);

        steps::state(&mut world, level.width(), dt);

        if get_time() - elapsed > 0.128 {
            elapsed = get_time();
            steps::animation(&mut world);
        }

        let pos = steps::position_for(&mut world, first_entity);

        cam.set_default_cam();
        cam.update(dt, &mut world, first_entity, pos, &mut level.tiles);

        draw_debug_info(total_werfs, cam.mpos, level.width());

        next_frame().await
    }
}

fn draw_debug_info(total_werfs: i32, mouse_pos: Vec2, world_width: usize) {
    macroquad_profiler::profiler(macroquad_profiler::ProfilerParams {
        fps_counter_pos: Vec2 {
            x: 16.0,
            y: screen_height() - 116.0,
        },
    });

    draw_text(
        format!("WERFS: {}", total_werfs).as_str(),
        16.0,
        16.0,
        16.0,
        WHITE,
    );

    draw_text(
        format!(
            "MOUSE: {{x: {}, y: {}}}, {}, {{x: {}, y: {}}}",
            mouse_pos.x as i32,
            mouse_pos.y as i32,
            v2_to_index(mouse_pos, world_width),
            (mouse_pos.x * TILE_SIZE) as i32,
            (mouse_pos.y * TILE_SIZE) as i32,
        )
        .as_str(),
        16.0,
        32.0,
        16.0,
        WHITE,
    );
}
