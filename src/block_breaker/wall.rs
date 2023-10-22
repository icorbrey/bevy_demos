use bevy::prelude::*;

use super::sprite_sheets::SpriteSheets;

enum Wall {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

pub fn spawn_walls<const WIDTH: i32, const HEIGHT: i32>(
    sprite_sheets: Res<SpriteSheets>,
    mut commands: Commands,
) {
    let wall = |variant, x, y| SpriteSheetBundle {
        transform: Transform::from_translation(Vec3::new(32.0 * x as f32, 32.0 * y as f32, 0.0)),
        texture_atlas: sprite_sheets.walls.clone(),
        sprite: TextureAtlasSprite {
            index: match variant {
                Wall::Vertical => 0,
                Wall::Horizontal => 1,
                Wall::TopLeft => 2,
                Wall::TopRight => 3,
                Wall::BottomRight => 4,
                Wall::BottomLeft => 5,
            },
            ..default()
        },
        ..default()
    };

    let top = HEIGHT / 2;
    let left = -WIDTH / 2 - 1;
    let right = WIDTH / 2;
    let bottom = -HEIGHT / 2 - 1;

    for x in (left + 1)..right {
        commands.spawn_batch(vec![
            wall(Wall::Horizontal, x, top),
            wall(Wall::Horizontal, x, bottom),
        ]);
    }

    for y in (bottom + 1)..top {
        commands.spawn_batch(vec![
            wall(Wall::Vertical, left, y),
            wall(Wall::Vertical, right, y),
        ]);
    }

    commands.spawn_batch(vec![
        wall(Wall::TopLeft, left, top),
        wall(Wall::TopRight, right, top),
        wall(Wall::BottomRight, right, bottom),
        wall(Wall::BottomLeft, left, bottom),
    ])
}
