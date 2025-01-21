use crate::{constants, entities::State};
use std::process::exit;

use hecs::EntityBuilder;
use macroquad::prelude::*;
use pathfinding::prelude::bfs;

use crate::{
    constants::TILE_SIZE, entities::Position, tile::Tile, tiles::Tiles, utils::v2_to_index,
};

pub struct Camera {
    cam: Camera2D,
    vel: Vec2,
    zoom_vel: Vec2,
    pub mpos: Vec2,
}

impl Camera {
    pub fn new(pos: Vec2) -> Self {
        let zoom = 0.004;

        Self {
            cam: Camera2D {
                target: pos,
                zoom: vec2(zoom, zoom),
                ..Default::default()
            },
            vel: vec2(0., 0.),
            zoom_vel: vec2(0., 0.),
            mpos: vec2(0., 0.),
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        world: &mut hecs::World,
        entity: hecs::Entity,
        werf_pos: Option<Position>,
        tiles: &mut Tiles,
    ) {
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        let mut new_target = vec2(0., 0.);
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::J) {
            new_target.x = -1.;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            new_target.x = 1.;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::I) {
            new_target.y = -1.;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::K) {
            new_target.y = 1.;
        }

        self.vel += new_target.normalize_or_zero();
        self.vel *= 0.85;

        let min_zoom = 0.001;
        let max_zoom = 0.008;

        let mut speed = 150.;
        speed *= ((min_zoom + max_zoom) - self.cam.zoom.x) * dt * 240.;
        self.cam.target += self.vel * speed;

        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                let new_zoom = y * 0.0005;
                self.zoom_vel += new_zoom;
            }
            _ => (),
        }

        self.zoom_vel *= 0.55;
        self.cam.zoom += self.zoom_vel;
        self.cam.zoom = self
            .cam
            .zoom
            .clamp(vec2(min_zoom, min_zoom), vec2(max_zoom, max_zoom));

        let mut pos = self.cam.screen_to_world(mouse_position().into());
        pos = pos / TILE_SIZE;
        pos = pos.max(vec2(0., 0.));
        pos = pos.min(vec2((tiles.width as f32) - 1., (tiles.width as f32) - 1.));
        pos = pos.abs();
        self.mpos = pos;

        if is_mouse_button_pressed(MouseButton::Left) {
            let Some(werf_pos) = werf_pos else {
                return;
            };

            let mpos_idx = v2_to_index(self.mpos, tiles.width);

            let Some(path) = bfs(
                &werf_pos.to_world_index(tiles.width, true),
                |p| p.successors(tiles),
                |p| p.0 == mpos_idx as i32,
            ) else {
                return;
            };

            if constants::DEBUG_MOUSE_CLICK {
                for i in 0..tiles.tiles.len() {
                    if tiles.tiles[i] == Tile::Red {
                        tiles.tiles[i] = Tile::Ground01;
                    }
                }

                for idx in path.clone() {
                    tiles.tiles[idx.0 as usize] = Tile::Red;
                }
            }

            let mut builder = EntityBuilder::new();
            builder.add(State::new_moving(path));

            // This should always succeed, hence panic.
            world
                .insert(entity, builder.build())
                .expect("failed to insert moving state");
        }
    }

    pub fn set_cam(&mut self, r: Option<RenderTarget>) {
        self.cam.render_target = r;
        set_camera(&self.cam);
    }

    pub fn set_default_cam(&self) {
        set_default_camera();
    }
}
