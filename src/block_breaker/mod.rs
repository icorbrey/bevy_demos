mod camera;
mod tileset;
mod wall;

use bevy::prelude::*;

use self::{camera::CameraPlugin, wall::WallPlugin};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CameraPlugin, WallPlugin))
        .run()
}
