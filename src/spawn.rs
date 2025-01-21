use ::rand::{rngs::ThreadRng, Rng};
use hecs::{Entity, World};
use macroquad::prelude::*;

use crate::{
    constants::TILE_SIZE,
    entities::{Animated, Position, State, Velocity},
};

pub fn two_werfs(counter: &mut i32, world: &mut World, rng: &mut ThreadRng) -> Entity {
    let main = world.spawn(werf(
        counter,
        Vec2 {
            x: screen_width() / 5.0 - TILE_SIZE * 8.0,
            y: screen_height() / 5.0,
        },
        // Vec2 { x: 0.7, y: 0.0 },
        Vec2 { x: 0.0, y: 0.0 },
        rng.gen_range(0..=3),
    ));
    world.spawn(werf(
        counter,
        Vec2 {
            x: screen_width() / 5.0 + TILE_SIZE * 8.0,
            y: screen_height() / 5.0,
        },
        // Vec2 { x: -0.7, y: 0.0 },
        Vec2 { x: 0.0, y: 0.0 },
        rng.gen_range(0..=3),
    ));
    main
}

#[allow(dead_code)]
pub fn many_werfs(counter: &mut i32, world: &mut World, amount: usize, rng: &mut ThreadRng) {
    world.spawn_batch((0..amount).map(|_| {
        werf(
            counter,
            Vec2 {
                x: rng.gen_range(TILE_SIZE..screen_width() / 5.0 - TILE_SIZE * 2.0),
                y: rng.gen_range(TILE_SIZE..screen_height() / 5.0 - TILE_SIZE * 2.0),
            },
            Vec2 {
                x: rng.gen_range(-0.7..0.7),
                y: rng.gen_range(-0.7..0.7),
            },
            rng.gen_range(0..=3),
        )
    }));
}

pub fn werf(
    counter: &mut i32,
    p: Vec2,
    v: Vec2,
    sprite: u8,
) -> (Position, Velocity, Animated, State) {
    *counter += 1;
    (
        Position { p },
        Velocity { v },
        Animated { sprite, step: 0 },
        State::Idle,
    )
}
