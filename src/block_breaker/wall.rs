use bevy::prelude::*;

use super::{coordinate::Coordinate, tileset::Tileset};

const WALL_SIZE: (i32, i32) = (32, 24);

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, WallAsset::load)
            .add_systems(Update, spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands, wall_asset: Res<WallAsset>) {
    let (w, h) = WALL_SIZE;

    let top = 32 * h / 2 - 16;
    let left = -32 * w / 2 + 16;
    let right = 32 * w / 2 - 16;
    let bottom = -32 * h / 2 + 16;

    for x in 1..(w as i32 - 1) {
        commands.spawn(wall_asset.tile(WallTile::Horizontal, Coordinate::new(left + x, top)));
        commands.spawn(wall_asset.tile(WallTile::Horizontal, Coordinate::new(left + x, bottom)));
    }

    for y in 1..(h as i32 - 1) {
        commands.spawn(wall_asset.tile(WallTile::Vertical, Coordinate::new(left, top - y)));
        commands.spawn(wall_asset.tile(WallTile::Vertical, Coordinate::new(right, top - y)));
    }

    commands.spawn_batch(vec![
        wall_asset.tile(WallTile::TopLeft, Coordinate::new(left, top)),
        wall_asset.tile(WallTile::TopRight, Coordinate::new(right, top)),
        wall_asset.tile(WallTile::BottomRight, Coordinate::new(right, bottom)),
        wall_asset.tile(WallTile::BottomLeft, Coordinate::new(left, bottom)),
    ]);
}

pub enum WallTile {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Resource)]
pub struct WallAsset(pub Handle<TextureAtlas>);

impl WallAsset {
    pub fn tile(
        &self,
        tile: WallTile,
        coordinate: Coordinate<{ WallAsset::TILE_SIZE }>,
    ) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: match tile {
                    WallTile::Vertical => 0,
                    WallTile::Horizontal => 1,
                    WallTile::TopLeft => 2,
                    WallTile::TopRight => 3,
                    WallTile::BottomRight => 4,
                    WallTile::BottomLeft => 5,
                },
                ..default()
            },
            texture_atlas: self.0.clone(),
            transform: Transform::from_translation(coordinate.into_vec2().extend(0.0)),
            ..default()
        }
    }
}

impl Tileset for WallAsset {
    const PATH: &'static str = "tilesets/block-breaker-wall.png";
    const TILE_SIZE: usize = 32;
    const COLUMNS: usize = 3;
    const ROWS: usize = 2;

    fn new(tileset: Handle<TextureAtlas>) -> Self {
        Self(tileset)
    }
}
