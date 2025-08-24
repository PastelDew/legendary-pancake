use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum SceneStatus {
    #[default]
    Main,
    InGame,
    GameOver,
}
