mod camera;
mod resources;

use bevy::prelude::*;

use self::{camera::CameraPlugin, resources::ResourcePlugin};

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
        .add_plugins((CameraPlugin, ResourcePlugin))
        .run();
}
