mod game;
mod modules;

use bevy::prelude::*;
use game::scenes::main_scene::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainScene)
        .run();
}
