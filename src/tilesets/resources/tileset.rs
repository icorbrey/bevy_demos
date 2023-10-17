use bevy::prelude::*;

pub trait Tileset
where
    Self: Sized + Resource,
{
    const PATH: &'static str;
    const TILE_SIZE: Vec2;
    const COLUMNS: usize;
    const ROWS: usize;

    fn new(tileset: Handle<TextureAtlas>) -> Self;

    fn load(
        mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let tileset = asset_server.load(Self::PATH);
        let tileset = TextureAtlas::from_grid(
            tileset,
            Self::TILE_SIZE,
            Self::COLUMNS,
            Self::ROWS,
            None,
            None,
        );
        let tileset = texture_atlasses.add(tileset);

        commands.insert_resource(Self::new(tileset));
    }
}
