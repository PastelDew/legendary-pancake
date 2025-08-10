use super::scene_states::SceneStatus;
use bevy::ecs::schedule::SystemConfigs;

pub trait IScene: Send + Sync + 'static {
    fn state(&self) -> SceneStatus;
    fn system_on_enter(&self) -> SystemConfigs;
    fn system_on_update(&self) -> SystemConfigs;
    fn system_on_exit(&self) -> SystemConfigs;
}