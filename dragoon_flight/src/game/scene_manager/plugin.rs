use super::app_extensions::AppSceneExtensions;
use super::scene_states::SceneStatus;
use super::scenes::{ingame_scene::InGameScene, main_scene::MainScene};
use bevy::prelude::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SceneStatus::Main)
            .add_scene(MainScene {})
            .add_scene(InGameScene {});
    }
}
