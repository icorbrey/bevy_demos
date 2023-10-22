use bevy::{asset::LoadState, prelude::*};

use super::{sprite_sheets::SpriteSheets, sprites::Sprites, AppState};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetManager::new())
            .add_systems(
                OnEnter(AppState::Loading),
                (Sprites::load, SpriteSheets::load),
            )
            .add_systems(
                Update,
                AssetManager::check_for_completion.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Resource)]
pub struct AssetManager {
    handles: Vec<HandleUntyped>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { handles: vec![] }
    }

    pub fn check_for_completion(
        mut next_state: ResMut<NextState<AppState>>,
        asset_manager: Res<AssetManager>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let handles = asset_manager.handles.iter().map(|h| h.id());

        match asset_server.get_group_load_state(handles) {
            LoadState::Loaded => {
                commands.remove_resource::<AssetManager>();
                next_state.set(AppState::Running)
            }
            _ => {}
        }
    }

    pub fn load(&mut self, asset_server: Res<AssetServer>, path: &str) -> Handle<Image> {
        let handle = asset_server.load(path);
        self.handles.push(handle.clone_untyped());
        handle
    }
}
