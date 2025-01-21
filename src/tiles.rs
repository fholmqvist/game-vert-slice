use ::rand::{rngs::ThreadRng, Rng};
use macroquad::prelude::*;

use crate::{
    constants::{TILESHEET_WIDTH, TILE_SIZE},
    tile::Tile,
    utils::xy_to_index,
};

#[derive(Debug)]
pub struct Tiles {
    pub tiles: Vec<Tile>,
    pub width: usize,
}

#[allow(dead_code)]
impl Tiles {
    pub fn new(rng: &mut ThreadRng, tiles: Vec<u8>, width: usize) -> Self {
        let tiles = tiles
            .iter()
            .map(|t| match t {
                0 => Tile::Ground01,
                1 => Tile::WallTop01,
                _ => Tile::Ground01,
            })
            .collect::<Vec<_>>();

        let mut s = Self { tiles, width };

        for i in 0..s.tiles.len() {
            s.update_tile(rng, i);
        }

        s
    }

    pub fn update_tile(&mut self, rng: &mut ThreadRng, index: usize) {
        if !self.tile_is_wall(index) {
            return;
        }

        self.tiles[index] = self.random_wall_top(rng);

        if let Some(b) = self.tile_below(index) {
            if b.is_ground() {
                self.tiles[index] = self.random_wall_side(rng);
            }
        }
    }

    pub fn update_neighbours(&mut self, rng: &mut ThreadRng, index: usize) {
        if self.tiles[index].is_wall() {
            return;
        }

        self.tiles[index] = self.random_ground(rng);

        if let Some(top) = self.tile_above(index) {
            if top.is_wall() {
                self.tiles[index - self.width] = self.random_wall_side(rng);
            }
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = (i % self.width) as f32 * TILE_SIZE;
            let y = (i / self.width) as f32 * TILE_SIZE;
            let tile_i = *tile as u8;
            let tileset_x = (tile_i % TILESHEET_WIDTH as u8) as f32 * TILE_SIZE;
            let tileset_y = (tile_i / TILESHEET_WIDTH as u8) as f32 * TILE_SIZE;
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    }),
                    source: Some(Rect {
                        x: tileset_x,
                        y: tileset_y,
                        w: TILE_SIZE,
                        h: TILE_SIZE,
                    }),
                    ..Default::default()
                },
            );
        }
    }

    pub fn set_square(&mut self, rng: &mut ThreadRng, x: usize, y: usize, size: usize, t: Tile) {
        for x in x..x + size {
            for y in y..y + size {
                self.tiles[x + y * self.width] = t;
            }
        }
        for x in x..x + size {
            for y in y..y + size {
                let index = xy_to_index(x, y, self.width);
                self.update_neighbours(rng, index);
            }
        }
    }

    pub fn tile_above(&self, index: usize) -> Option<Tile> {
        if index < self.width {
            return None;
        }
        let index = index - self.width;
        if index >= self.tiles.len() {
            return None;
        }
        Some(self.tiles[index])
    }

    pub fn tile_below(&self, index: usize) -> Option<Tile> {
        let index = index + self.width;
        if index >= self.tiles.len() {
            return None;
        }
        Some(self.tiles[index])
    }

    pub fn tile_at_right(&self, index: usize) -> Option<Tile> {
        let index = index + 1;
        if index >= self.tiles.len() {
            return None;
        }
        Some(self.tiles[index])
    }

    pub fn tile_at_left(&self, index: usize) -> Option<Tile> {
        if index == 0 {
            return None;
        }
        let index = index - 1;
        if index >= self.tiles.len() {
            return None;
        }
        Some(self.tiles[index])
    }

    fn tile_is_wall(&self, index: usize) -> bool {
        let tile = self.tiles[index];
        tile.is_wall()
    }

    fn random_ground(&self, rng: &mut ThreadRng) -> Tile {
        if rng.gen_ratio(90, 100) {
            return Tile::Ground01;
        }

        match rng.gen_range(0..=3) {
            0 => Tile::Ground01,
            1 => Tile::Ground02,
            2 => Tile::Ground03,
            _ => Tile::Ground01,
        }
    }

    fn random_wall_top(&self, rng: &mut ThreadRng) -> Tile {
        if rng.gen_ratio(85, 100) {
            return Tile::WallTop01;
        }

        match rng.gen_range(0..=5) {
            0 => Tile::WallTop01,
            1 => Tile::WallTop02,
            2 => Tile::WallTop03,
            3 => Tile::WallTop04,
            4 => Tile::WallTop05,
            _ => Tile::WallTop01,
        }
    }

    fn random_wall_side(&self, rng: &mut ThreadRng) -> Tile {
        if rng.gen_ratio(85, 100) {
            return Tile::WallSide01;
        }

        match rng.gen_range(0..=5) {
            0 => Tile::WallSide01,
            1 => Tile::WallSide02,
            2 => Tile::WallSide03,
            3 => Tile::WallSide04,
            4 => Tile::WallSide05,
            _ => Tile::WallSide01,
        }
    }
}
