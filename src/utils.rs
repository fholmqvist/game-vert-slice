use macroquad::math::Vec2;

pub fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

pub fn v2_to_index(v: Vec2, width: usize) -> usize {
    let x = v.x as usize;
    let y = v.y as usize;
    xy_to_index(x, y, width)
}

pub fn index_to_v2(idx: i32, width: usize) -> Vec2 {
    let x = idx % width as i32;
    let y = idx / width as i32;
    Vec2 {
        x: x as f32,
        y: y as f32,
    }
}
