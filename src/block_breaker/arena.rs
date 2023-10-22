use bevy::prelude::*;

use super::{background::spawn_background, wall::spawn_walls, AppState};

const WIDTH: i32 = 24;
const HEIGHT: i32 = 16;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Running),
            (
                spawn_background::<WIDTH, HEIGHT>,
                spawn_walls::<WIDTH, HEIGHT>,
            ),
        );
    }
}
