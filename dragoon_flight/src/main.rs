mod game;

use bevy::prelude::*;
use game::scene_manager::plugin::ScenesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .run();
}
