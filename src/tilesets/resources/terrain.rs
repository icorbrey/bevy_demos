use bevy::prelude::*;

use super::tileset::LoadableTileset;

#[derive(Resource)]
pub struct Terrain(pub Handle<TextureAtlas>);

impl LoadableTileset for Terrain {
    fn new(tileset: Handle<TextureAtlas>) -> Self {
        Self(tileset)
    }
}
