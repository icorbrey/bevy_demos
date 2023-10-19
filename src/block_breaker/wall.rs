use bevy::prelude::*;

use super::tileset::Tileset;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, WallAsset::load)
            .add_systems(Update, spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands, asset: Res<WallAsset>) {
    commands.spawn_batch(
        (0..10)
            .map(|x| {
                asset.get_sprite(
                    WallTile::Horizontal,
                    Transform::from_translation(Vec3::new(160.0 - 32.0 * x as f32, 0.0, 0.0)),
                )
            })
            .collect::<Vec<SpriteSheetBundle>>(),
    );
}

#[derive(Resource)]
pub struct WallAsset(pub Handle<TextureAtlas>);

pub enum WallTile {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl Into<usize> for WallTile {
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

impl WallAsset {
    pub fn get_sprite(&self, variant: WallTile, transform: Transform) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: self.0.clone(),
            transform,
            sprite: TextureAtlasSprite {
                index: variant.into(),
                ..default()
            },
            ..default()
        }
    }
}

impl Tileset for WallAsset {
    const PATH: &'static str = "tilesets/block-breaker-wall.png";
    const TILE_SIZE: bevy::prelude::Vec2 = Vec2::splat(32.0);
    const COLUMNS: usize = 3;
    const ROWS: usize = 2;

    fn new(tileset: Handle<TextureAtlas>) -> Self {
        Self(tileset)
    }
}
