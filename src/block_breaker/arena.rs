use bevy::prelude::*;

use super::{spawn::SpawnEvent, wall::WallVariant, AppState};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Running), build_arena);
    }
}

fn build_arena(mut ev_spawn: EventWriter<SpawnEvent>) {
    let width = 24;
    let height = 16;

    let left = -width / 2 - 1;
    let bottom = -height / 2 - 1;

    ev_spawn.send_batch(
        (0..width)
            .flat_map(|x| (0..height).map(move |y| SpawnEvent::Background(left + x, bottom + y))),
    );

    ev_spawn.send_batch((0..width).flat_map(|x| {
        vec![
            SpawnEvent::Wall(WallVariant::Horizontal, left + x, bottom + height),
            SpawnEvent::Wall(WallVariant::Horizontal, left + x, bottom - 1),
        ]
    }));

    ev_spawn.send_batch((0..height).flat_map(|y| {
        vec![
            SpawnEvent::Wall(WallVariant::Vertical, left - 1, bottom + y),
            SpawnEvent::Wall(WallVariant::Vertical, left + width, bottom + y),
        ]
    }));

    ev_spawn.send_batch(vec![
        SpawnEvent::Wall(WallVariant::TopLeft, left - 1, bottom + height),
        SpawnEvent::Wall(WallVariant::TopRight, left + width, bottom + height),
        SpawnEvent::Wall(WallVariant::BottomRight, left + width, bottom - 1),
        SpawnEvent::Wall(WallVariant::BottomLeft, left - 1, bottom - 1),
    ]);

    ev_spawn.send(SpawnEvent::Ball(Vec2::splat(0.0), Vec2::splat(0.0)));
}
