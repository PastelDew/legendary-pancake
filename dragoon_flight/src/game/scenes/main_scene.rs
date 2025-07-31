use bevy::prelude::*;
pub struct MainScene;

impl MainScene {
    fn on_update() {
        info!("TEST!!!");
    }
}

impl Plugin for MainScene {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, Self::on_update);
    }
}
