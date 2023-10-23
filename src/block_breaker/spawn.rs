use bevy::prelude::*;

use super::wall::WallVariant;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>();
    }
}

#[derive(Event)]
pub enum SpawnEvent {
    Ball(Vec2, Vec2),
    Background(i32, i32),
    Wall(WallVariant, i32, i32),
}
