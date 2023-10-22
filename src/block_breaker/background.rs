use bevy::prelude::*;

use super::sprites::Sprites;

pub fn spawn_background<const WIDTH: i32, const HEIGHT: i32>(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    let top = HEIGHT / 2;
    let left = -WIDTH / 2;
    let right = WIDTH / 2;
    let bottom = -HEIGHT / 2;

    for x in left..right {
        for y in bottom..top {
            commands.spawn(SpriteBundle {
                texture: sprites.background.clone(),
                transform: Transform::from_translation(Vec3::new(
                    32.0 * x as f32,
                    32.0 * y as f32,
                    0.0,
                )),
                ..default()
            });
        }
    }
}
