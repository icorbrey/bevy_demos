mod terrain;
mod tileset;

use bevy::prelude::*;

use self::{terrain::Terrain, tileset::Tileset};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            Tileset::<Terrain>::load(
                "tilesets/grass-stone-simple.png",
                Vec2::new(32.0, 32.0),
                (5, 1),
            ),
        );
    }
}
