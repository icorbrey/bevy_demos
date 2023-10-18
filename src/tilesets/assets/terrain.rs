use bevy::prelude::*;

use super::tileset::Tileset;

#[derive(Resource)]
pub struct Terrain(pub Handle<TextureAtlas>);

impl Tileset for Terrain {
    const PATH: &'static str = "tilesets/grass-stone-simple.png";
    const TILE_SIZE: Vec2 = Vec2::splat(32.0);
    const COLUMNS: usize = 5;
    const ROWS: usize = 1;

    fn new(tileset: Handle<TextureAtlas>) -> Self {
        Self(tileset)
    }
}
