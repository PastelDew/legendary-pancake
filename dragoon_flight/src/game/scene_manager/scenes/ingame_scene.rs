use bevy::{ecs::schedule::*, prelude::*};
use super::super::scene_states::SceneStatus;
use super::super::scene_traits::*;
use crate::game::anime::{AnimationPlaybackState, spawn_animated_svg}; // 올바른 임포트 경로

pub struct InGameScene {}

impl IScene for InGameScene {
    fn state(&self) -> SceneStatus {
        SceneStatus::InGame
    }

    fn system_on_enter(&self) -> SystemConfigs {
        // 애니메이션 엔티티를 생성하는 시스템 호출
        spawn_game_entities.into_configs()
    }

    fn system_on_update(&self) -> SystemConfigs {
        on_update.into_configs()
    }

    fn system_on_exit(&self) -> SystemConfigs {
        on_exit.into_configs()
    }
}

// 게임 엔티티들을 생성하는 시스템
fn spawn_game_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 주인공 애니메이션 생성
    spawn_animated_svg(
        &mut commands,
        &asset_server,
        vec![
            "anime/protagonist_1.svg".to_string(),
            "anime/protagonist_2.svg".to_string(),
            "anime/protagonist_3.svg".to_string(),
        ],
        0.1, // 프레임 지속 시간 (초)
        AnimationPlaybackState::Playing, // 초기 재생 상태
        Vec3::new(-200.0, 0.0, 0.0), // 위치 (x, y, z)
        Vec3::new(1.0, 1.0, 1.0), // 스케일 (x, y, z)
    );

    // 적 캐릭터 애니메이션 생성
    spawn_animated_svg(
        &mut commands,
        &asset_server,
        vec![
            "anime/enemy_1.svg".to_string(),
            "anime/enemy_2.svg".to_string(),
            "anime/enemy_3.svg".to_string(),
        ],
        0.15, // 프레임 지속 시간 (초)
        AnimationPlaybackState::Playing,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );

    // 총알 애니메이션 생성
    spawn_animated_svg(
        &mut commands,
        &asset_server,
        vec![
            "anime/bullet_1.svg".to_string(),
            "anime/bullet_2.svg".to_string(),
            "anime/bullet_3.svg".to_string(),
        ],
        0.05, // 프레임 지속 시간 (초)
        AnimationPlaybackState::Playing,
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );
}

// 기존 on_update와 on_exit 함수는 그대로 유지
fn on_update() {}
fn on_exit() {}