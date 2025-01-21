use kd_tree::KdPoint;
use macroquad::prelude::*;

use crate::{constants::TILE_SIZE, tiles::Tiles, utils::index_to_v2};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub p: Vec2,
}

impl Position {
    pub fn to_world_index(&self, world_width: usize, divide: bool) -> WorldIndex {
        let mut x = self.p.x as i32;
        let mut y = self.p.y as i32;
        if divide {
            x = x / TILE_SIZE as i32;
            y = y / TILE_SIZE as i32;
        }

        let idx = x + y * world_width as i32;

        WorldIndex(idx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldIndex(pub i32);

impl WorldIndex {
    // Currently only returns cardinal directions.
    // Future implementation will include diagonal
    // directions, however these require an
    // accompanied cost as to compensate for the
    // extra distance they cover.
    pub fn successors(&self, tiles: &Tiles) -> Vec<WorldIndex> {
        let mut available = vec![];

        if let Some(top) = tiles.tile_above(self.0 as usize) {
            if !top.is_blocked() {
                available.push(WorldIndex(self.0 - tiles.width as i32));
            }
        }

        if let Some(right) = tiles.tile_at_right(self.0 as usize) {
            if !right.is_blocked() {
                available.push(WorldIndex(self.0 + 1 as i32));
            }
        }

        if let Some(bottom) = tiles.tile_below(self.0 as usize) {
            if !bottom.is_blocked() {
                available.push(WorldIndex(self.0 + tiles.width as i32));
            }
        }

        if let Some(left) = tiles.tile_at_left(self.0 as usize) {
            if !left.is_blocked() {
                available.push(WorldIndex(self.0 - 1 as i32));
            }
        }

        available
    }

    pub fn to_vec(&self, width: usize) -> Vec2 {
        index_to_v2(self.0, width)
    }
}

// Trait for inserting into a kd-tree.
impl KdPoint for Position {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.p.x;
        }
        self.p.y
    }
}

pub struct Velocity {
    pub v: Vec2,
}

pub struct Animated {
    pub sprite: u8,
    pub step: u8,
}

#[derive(Debug)]
pub struct Moving {
    path: Vec<WorldIndex>,
    curr: usize,
}

impl Moving {
    pub fn update(&mut self, world_width: usize, pos: &Position, vel: &mut Velocity, dt: f32) -> bool {
        if self.curr >= self.path.len() {
            return true;
        }

        let target = self.path[self.curr].to_vec(world_width) * TILE_SIZE;
        let dir = target - pos.p;

        if dir.length_squared().abs() < 50.0 {
            self.curr += 1;
            if self.curr >= self.path.len() {
                return true;
            }
        }

        let speed = dir.normalize() * dt * 4.0;
        vel.v += speed;

        return false;
    }
}

#[derive(Debug)]
pub enum State {
    Idle,
    Moving(Moving),
}

impl State {
    pub fn new_moving(path: Vec<WorldIndex>) -> State {
        Self::Moving(Moving { path, curr: 0 })
    }

    pub fn update(&mut self, world_width: usize, pos: &Position, vel: &mut Velocity, dt: f32) {
        match self {
            State::Idle => (),
            State::Moving(moving) => {
                let finished = moving.update(world_width, pos, vel, dt);
                if finished {
                    *self = State::Idle;
                }
            }
        }
    }
}
