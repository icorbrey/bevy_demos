use bevy::prelude::*;

use super::{spawn::SpawnEvent, sprites::Sprites};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_background);
    }
}

pub fn spawn_background(
    mut ev_spawn: EventReader<SpawnEvent>,
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    for ev in ev_spawn.iter() {
        if let SpawnEvent::Background(x, y) = ev {
            commands.spawn(SpriteBundle {
                texture: sprites.test_grey.clone(),
                transform: Transform::from_translation(Vec3::new(
                    32.0 * *x as f32 + 48.0,
                    32.0 * *y as f32 + 48.0,
                    0.0,
                )),
                ..default()
            });
        }
    }
}
