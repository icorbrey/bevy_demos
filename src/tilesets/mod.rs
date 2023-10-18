mod assets;
mod camera;

use bevy::prelude::*;

use self::{assets::AssetPlugin, camera::CameraPlugin};

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tileset Demo".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((CameraPlugin, AssetPlugin))
        .run();
}
