use bevy::prelude::*;

use super::{spawn::SpawnEvent, sprites::Sprites};

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ball);
    }
}

pub fn spawn_ball(
    mut ev_spawn: EventReader<SpawnEvent>,
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    for ev in ev_spawn.iter() {
        if let SpawnEvent::Ball(position, _) = ev {
            commands.spawn(SpriteBundle {
                texture: sprites.ball.clone(),
                transform: Transform::from_translation(position.extend(1.0)),
                ..default()
            });
        }
    }
}
