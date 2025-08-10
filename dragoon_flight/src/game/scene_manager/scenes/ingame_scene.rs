use super::super::scene_traits::*;
use super::super::scene_states::SceneStatus;
use bevy::{ecs::schedule::*, prelude::*};

pub struct InGameScene {}

impl InGameScene {
    pub fn on_enter() {
        info!("On InGameScene Enter");
    }

    pub fn on_update() {
        info!("On InGameScene Update");
    }

    pub fn on_exit() {
        info!("On InGameScene Exit");
    }
}

impl IScene for InGameScene {
    fn state(&self) -> SceneStatus {
        SceneStatus::InGame
    }

    fn system_on_enter(&self) -> SystemConfigs {
        Self::on_enter.into_configs()
    }

    fn system_on_update(&self) -> SystemConfigs {
        Self::on_update.into_configs()
    }

    fn system_on_exit(&self) -> SystemConfigs {
        Self::on_exit.into_configs()
    }
}
