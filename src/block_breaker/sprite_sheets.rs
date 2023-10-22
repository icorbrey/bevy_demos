use bevy::prelude::*;

use super::assets::AssetManager;

#[derive(Resource)]
pub struct SpriteSheets {
    pub walls: Handle<TextureAtlas>,
}

impl SpriteSheets {
    pub fn load(
        mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
        mut asset_manager: ResMut<AssetManager>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let mut load = |path, size, w, h| {
            let tileset = asset_manager.load(&asset_server, path);
            let size = Vec2::splat(size as f32);
            let tileset = TextureAtlas::from_grid(tileset, size, w, h, None, None);
            texture_atlasses.add(tileset)
        };

        commands.insert_resource(SpriteSheets {
            walls: load("tilesets/block-breaker-wall.png", 32, 3, 2),
        })
    }
}
