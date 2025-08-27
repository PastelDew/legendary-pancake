use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use super::super::{scene_states::SceneStatus, scene_traits::IScene};

pub struct GameOverScene;

impl IScene for GameOverScene {
    fn state(&self) -> SceneStatus { SceneStatus::GameOver }

    fn system_on_enter(&self) -> SystemConfigs { setup_game_over.into_configs() }

    fn system_on_update(&self) -> SystemConfigs { game_over_interaction.into_configs() }

    fn system_on_exit(&self) -> SystemConfigs { despawn_screen::<OnGameOverScreen>.into_configs() }
}

#[derive(Component)]
struct OnGameOverScreen;

fn setup_game_over(mut commands: Commands) {
    commands.spawn((Camera2d, OnGameOverScreen));

    commands
        .spawn((
            OnGameOverScreen,
            Node { justify_content: JustifyContent::Center, align_items: AlignItems::Center, width: Val::Percent(100.0), height: Val::Percent(100.0), ..Default::default() },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over\nPress Enter to return"),
                TextFont { font_size: 48.0, ..Default::default() },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

fn game_over_interaction(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<SceneStatus>>) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        next.set(SceneStatus::Main);
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for e in &to_despawn { commands.entity(e).despawn_recursive(); }
}
