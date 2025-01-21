use std::{fs::read_to_string, io};

use ::rand::rngs::ThreadRng;
use macroquad::prelude::*;

use crate::tiles::Tiles;

#[derive(Debug)]
pub struct Level {
    pub tiles: Tiles,
}

impl Level {
    pub fn new(rng: &mut ThreadRng, path: &str) -> io::Result<Self> {
        let (tiles, width) = load_level(path)?;

        Ok(Self {
            tiles: Tiles::new(rng, tiles, width),
        })
    }

    pub fn draw(&self, texture: &Texture2D) {
        self.tiles.draw(texture);
    }

    pub fn width(&self) -> usize {
        self.tiles.width
    }
}

fn load_level(path: &str) -> io::Result<(Vec<u8>, usize)> {
    let mut width = 0;
    let tiles = read_to_string(path)?
        .lines()
        .flat_map(|line| {
            if width == 0 {
                // Digits are 2 characters wide,
                // divided by spaces, no space
                // at the end.
                width = (line.len() / 3) + 1;
            }
            line.split(" ")
                .map(|c| c.parse::<u8>().unwrap_or(0))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    Ok((tiles, width))
}
