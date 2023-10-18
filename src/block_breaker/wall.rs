use bevy::prelude::*;

use super::tileset::Tileset;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, WallAsset::load);
    }
}

#[derive(Resource)]
pub struct WallAsset(pub Handle<TextureAtlas>);

impl Tileset for WallAsset {
    const PATH: &'static str = "tilesets/block-breaker-wall.png";
    const TILE_SIZE: bevy::prelude::Vec2 = Vec2::splat(16.0);
    const COLUMNS: usize = 3;
    const ROWS: usize = 3;

    fn new(tileset: Handle<TextureAtlas>) -> Self {
        Self(tileset)
    }
}
