use super::scene_traits::IScene;
use bevy::prelude::*;

pub trait AppSceneExtensions {
    fn add_scene<T: IScene + 'static>(&mut self, scene: T) -> &mut Self;
}

impl AppSceneExtensions for App {
    fn add_scene<T: IScene + 'static>(&mut self, scene: T) -> &mut Self {
        let state = scene.state();
        self.add_systems(OnEnter(state), scene.system_on_enter())
            .add_systems(OnExit(state), scene.system_on_exit())
            .add_systems(Update, scene.system_on_update().run_if(in_state(state)));
        self
    }
}