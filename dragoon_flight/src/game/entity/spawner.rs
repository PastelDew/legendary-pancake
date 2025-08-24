use super::anime::*;
use super::entity_properties::*;
use bevy::prelude::Entity;
use bevy::prelude::*;

pub fn spawn_entity(
    commands: &mut Commands,
    anime: Animation,
    position: Vec3, // 엔티티의 초기 위치
    scale: Vec3,    // 엔티티의 초기 스케일
) -> Entity {
    let new_entity = spawn_animated_sprite(commands, anime, position, scale);
    commands
        .entity(new_entity)
        .insert(Velocity2D { x: 0.0, y: 0.0 })
        .insert(Acceleration2D { x: 0.0, y: 0.0 })
        .insert(Collider {
            offset: Vec3::ZERO,
            scale: Vec3::ONE,
        });
    new_entity
}

// let player_entity = spawn_animated_sprite(
//         &mut commands,
//         &asset_server,
//         vec![
//             "anime/protagonist_1.png".to_string(),
//             "anime/protagonist_2.png".to_string(),
//             "anime/protagonist_3.png".to_string(),
//         ],
//         0.1, // 프레임 지속 시간 (초)
//         AnimationPlaybackState::Playing, // 초기 재생 상태
//         Vec3::new(player_x, 0.0, 0.0), // 위치
//         Vec3::new(1.0, 1.0, 1.0), // 스케일
//     );
//     commands.entity(player_entity).insert(Player { speed: 300.0, fire_timer: Timer::from_seconds(0.15, TimerMode::Repeating) });
//     // 플레이어 체력 및 체력바(자식)
//     commands.entity(player_entity).insert(PlayerHealth { current: 3, max: 3 });
//     // 체력바 배경
//     let bar_offset = Vec3::new(0.0, 60.0, 0.2);
//     let bar_width = 80.0; let bar_height = 8.0;
//     commands.entity(player_entity).with_children(|p| {
//         p.spawn((
//             Sprite::from_color(Color::srgb(0.2, 0.2, 0.2), Vec2::new(bar_width, bar_height)),
//             Transform::from_translation(bar_offset),
//         ));
//         // 체력바 포그라운드
//         p.spawn((
//             Sprite::from_color(Color::srgb(0.2, 0.9, 0.2), Vec2::new(bar_width, bar_height)),
//             Transform::from_translation(bar_offset + Vec3::new(0.0, 0.0, 0.01)),
//             HealthBar { max_width: bar_width, height: bar_height },
//         ));
//     });
