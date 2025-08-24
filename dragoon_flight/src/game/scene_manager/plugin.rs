use crate::game::entity::anime::FrameCache;

use super::app_extensions::AppSceneExtensions;
use super::scene_states::SceneStatus;
use super::scenes::{
    game_over_scene::GameOverScene, ingame_scene::InGameScene, main_scene::MainScene,
};
use super::score::Score;
use bevy::prelude::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SceneStatus::Main)
            .add_scene(MainScene {})
            .add_scene(InGameScene {})
            .add_scene(GameOverScene);

        app.init_resource::<Score>().init_resource::<FrameCache>();
    }
}
