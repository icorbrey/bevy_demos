use std::marker::PhantomData;

use bevy::{asset::AssetPath, prelude::*};

pub struct Tileset<T: LoadableTileset> {
    phantom_data: PhantomData<T>,
}

impl<T: LoadableTileset + Resource> Tileset<T> {
    pub fn load<'a, P: Into<AssetPath<'a>> + Copy>(
        path: P,
        tile_size: Vec2,
        (columns, rows): (usize, usize),
    ) -> impl Fn(ResMut<Assets<TextureAtlas>>, Res<AssetServer>, Commands) -> () {
        move |mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
              asset_server: Res<AssetServer>,
              mut commands: Commands| {
            let tileset = asset_server.load(path);
            let tileset = TextureAtlas::from_grid(tileset, tile_size, columns, rows, None, None);
            let tileset = texture_atlasses.add(tileset);

            commands.insert_resource(T::new(tileset.clone()));
        }
    }
}

pub trait LoadableTileset {
    fn new(tileset: Handle<TextureAtlas>) -> Self;
}
