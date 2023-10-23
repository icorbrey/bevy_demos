use bevy::prelude::*;

use super::{spawn::SpawnEvent, sprites::Sprites};

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_wall);
    }
}

pub fn spawn_wall(
    mut ev_spawn: EventReader<SpawnEvent>,
    sprites: Res<Sprites>,
    mut commands: Commands,
) {
    for ev in ev_spawn.iter() {
        if let SpawnEvent::Wall(_, x, y) = ev {
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    32.0 * *x as f32 + 48.0,
                    32.0 * *y as f32 + 48.0,
                    0.0,
                )),
                texture: sprites.test_orange.clone(),
                ..default()
            });
        }
    }
}

#[derive(Clone, Copy)]
pub enum WallVariant {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl Into<usize> for WallVariant {
    fn into(self) -> usize {
        match self {
            Self::Vertical => 0,
            Self::Horizontal => 1,
            Self::TopLeft => 2,
            Self::TopRight => 3,
            Self::BottomRight => 4,
            Self::BottomLeft => 5,
        }
    }
}
