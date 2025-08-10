use super::super::scene_traits::*;
use super::super::scene_states::SceneStatus;
use bevy::{ecs::schedule::SystemConfigs, prelude::*};

pub struct MainScene {}

impl MainScene {
    fn on_enter() {}

    fn on_update() {}

    fn on_exit() {}
}

impl IScene for MainScene {
    fn state(&self) -> SceneStatus {
        SceneStatus::Main
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
