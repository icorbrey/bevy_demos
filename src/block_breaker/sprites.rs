use bevy::prelude::*;

use super::assets::AssetManager;

#[derive(Resource)]
pub struct Sprites {
    pub background: Handle<Image>,
    pub ball: Handle<Image>,
    pub test_grey: Handle<Image>,
    pub test_orange: Handle<Image>,
}

impl Sprites {
    pub fn load(
        mut asset_manager: ResMut<AssetManager>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let mut load = |path| asset_manager.load(&asset_server, path);

        commands.insert_resource(Self {
            background: load("sprites/block-breaker-bg.png"),
            ball: load("sprites/block-breaker-ball.png"),
            test_grey: load("sprites/test-32x32-grey.png"),
            test_orange: load("sprites/test-32x32-orange.png"),
        })
    }
}
