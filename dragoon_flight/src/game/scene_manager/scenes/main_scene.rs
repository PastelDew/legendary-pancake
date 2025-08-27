use bevy::{app::AppExit, ecs::schedule::SystemConfigs, prelude::*};

use super::super::{scene_states::SceneStatus, scene_traits::IScene};

// --- Constants ---
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// --- Marker Components ---
#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct StartGameButton;

#[derive(Component)]
struct ExitButton;

// --- Scene Definition ---
pub struct MainScene {}

impl IScene for MainScene {
    fn state(&self) -> SceneStatus {
        SceneStatus::Main
    }

    fn system_on_enter(&self) -> SystemConfigs {
        setup_main_menu.into_configs()
    }

    fn system_on_update(&self) -> SystemConfigs {
        main_menu_interaction.into_configs()
    }

    fn system_on_exit(&self) -> SystemConfigs {
        despawn_screen::<OnMainMenuScreen>.into_configs()
    }
}

// --- Systems ---
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D 카메라
    commands.spawn((Camera2d, OnMainMenuScreen));

    // 버튼 레이아웃(Node) 공통 스타일
    let button_node = Node {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    // 텍스트 공통 스타일 (프로젝트 폰트 사용)
    let text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let _font_regular: Handle<Font> = asset_server.load("fonts/NanumGothic.ttf");
    let font_bold: Handle<Font> = asset_server.load("fonts/NanumGothicBold.ttf");

    // 루트 컨테이너
    commands
        .spawn((
            OnMainMenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            // Start Game 버튼
            parent
                .spawn((
                    Button,
                    StartGameButton,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        TextFont { font: font_bold.clone(), font_size: 40.0, ..Default::default() },
                        text_color,
                    ));
                });

            // Exit 버튼
            parent
                .spawn((
                    Button,
                    ExitButton,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Exit"),
                        TextFont { font: font_bold, font_size: 40.0, ..Default::default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn main_menu_interaction(
    mut param_set: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartGameButton>)>, 
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>, 
    )>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<SceneStatus>>,
) {
    // Handle Start Game button
    for (interaction, mut color) in param_set.p0().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(SceneStatus::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    // Handle Exit button
    for (interaction, mut color) in param_set.p1().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_exit_events.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Generic system to despawn all entities with a given component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
