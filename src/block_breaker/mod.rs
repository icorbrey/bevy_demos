mod tileset;
mod wall;

use bevy::prelude::*;

use self::wall::WallPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WallPlugin)
        .run()
}
