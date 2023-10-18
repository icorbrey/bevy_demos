mod terrain;
mod tileset;

use bevy::prelude::*;

use self::{terrain::Terrain, tileset::Tileset};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Terrain::load);
    }
}
