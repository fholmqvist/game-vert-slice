use macroquad::prelude::*;

pub async fn load() -> (Texture2D, Texture2D) {
    let werfs_texture = load_texture("../resources/werfs.png")
        .await
        .expect("failed to load werf texture");

    werfs_texture.set_filter(FilterMode::Nearest);

    let tileset_texture = load_texture("../resources/tileset.png")
        .await
        .expect("failed to load tileset texture");

    tileset_texture.set_filter(FilterMode::Nearest);

    (werfs_texture, tileset_texture)
}
