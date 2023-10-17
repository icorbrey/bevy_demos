mod terrain;
mod tileset;

use bevy::prelude::*;

use self::{terrain::Terrain, tileset::Tileset};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Terrain::load);
    }
}
