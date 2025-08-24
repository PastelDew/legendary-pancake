mod game;

use bevy::prelude::*;
use game::scene_manager::plugin::ScenesPlugin;
use game::anime::AnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimationPlugin)
        .add_plugins(ScenesPlugin)
        .run();
}
