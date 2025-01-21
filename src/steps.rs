use crate::{
    constants::TILE_SIZE,
    entities::{Animated, Position, State, Velocity},
};

use hecs::World;
use macroquad::prelude::*;

pub fn move_and_draw(world: &mut World, texture: &Texture2D, positions: &mut Vec<Position>) {
    for (_id, (pos, vel, anim)) in world.query_mut::<(&mut Position, &mut Velocity, &Animated)>() {
        pos.p.x += vel.v.x;
        pos.p.y += vel.v.y;

        if vel.v.length_squared() < 0.001 {
            vel.v = vec2(0., 0.);
        } else {
            vel.v *= 0.96;
        }

        positions.push(Position { p: pos.p });

        draw_texture_ex(
            &texture,
            pos.p.x,
            pos.p.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: TILE_SIZE / 2.0,
                    y: TILE_SIZE / 2.0,
                }),
                source: Some(Rect {
                    x: anim.step as f32 * TILE_SIZE,
                    y: anim.sprite as f32 * TILE_SIZE,
                    w: TILE_SIZE,
                    h: TILE_SIZE,
                }),
                flip_x: vel.v.x < 0.0,
                ..Default::default()
            },
        );
    }
}

pub fn collision(world: &mut World, positions: Vec<Position>) {
    const RADIUS: f32 = TILE_SIZE / 4.0;
    let kdtree = kd_tree::KdTree::build_by_ordered_float(positions);

    for (_id, (pos, vel)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
        let nearest = kdtree.within_radius(pos, RADIUS * 2.0);
        let collision = nearest.len() > 1;

        if collision {
            let other = nearest[1];
            let mid_x = (pos.p.x + other.p.x) / 2.0;
            let mid_y = (pos.p.y + other.p.y) / 2.0;

            // Scales the distance to move the werf.
            // The idea is to have them collide but
            // then slide through each other.
            // This will have to be tuned.
            let setback = RADIUS * 1.8;
            // let setback = RADIUS * 1.74;

            pos.p.x = mid_x + RADIUS * (pos.p.x - other.p.x) / setback;
            pos.p.y = mid_y + RADIUS * (pos.p.y - other.p.y) / setback;

            vel.v *= 0.85;
        }

        draw_circle_lines(
            pos.p.x + RADIUS,
            pos.p.y + RADIUS,
            RADIUS,
            1.4,
            if collision { RED } else { GREEN },
        );
    }
}

pub fn animation(world: &mut World) {
    for (_id, (vel, anim)) in world.query_mut::<(&Velocity, &mut Animated)>() {
        if vel.v.length_squared() > 0.005 {
            anim.step = (anim.step + 1) % 2;
        } else {
            anim.step = 0;
        }
    }
}

pub fn state(world: &mut World, world_width: usize, dt: f32) {
    for (_id, (pos, vel, state)) in world.query_mut::<(&Position, &mut Velocity, &mut State)>() {
        state.update(world_width, pos, vel, dt);
    }
}

pub fn position_for(world: &mut hecs::World, entity: hecs::Entity) -> Option<Position> {
    match world.query_one_mut::<&Position>(entity) {
        Ok(pos) => Some(*pos),
        _ => None,
    }
}
