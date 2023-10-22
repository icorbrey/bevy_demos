mod arena;
mod assets;
mod background;
mod camera;
mod spawn;
mod sprite_sheets;
mod sprites;
mod wall;

use bevy::prelude::*;

use self::{
    arena::ArenaPlugin, assets::AssetPlugin, background::BackgroundPlugin, camera::CameraPlugin,
    spawn::SpawnPlugin, wall::WallsPlugin,
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_plugins(BackgroundPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(WallsPlugin)
        .run()
}
