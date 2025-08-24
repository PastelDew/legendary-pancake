use bevy::prelude::*;
use bevy_svg::prelude::*;

// 애니메이션 재생 상태를 정의하는 열거형
#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub enum AnimationPlaybackState {
    Playing,
    Paused,
    Stopped,
}

// 애니메이션의 데이터를 저장하는 컴포넌트
#[derive(Component)]
pub struct Animation {
    pub frames: Vec<Handle<Svg>>, // 애니메이션 프레임 SVG 핸들 목록
    pub timer: Timer,             // Bevy의 Timer를 사용하여 프레임 전환 관리
    pub current_frame_index: usize, // 현재 표시 중인 프레임 인덱스
    pub state: AnimationPlaybackState, // 애니메이션 재생 상태
}

// 애니메이션 엔티티를 생성하는 헬퍼 함수
pub fn spawn_animated_svg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    frame_paths: Vec<String>, // 애니메이션 프레임 파일 경로 목록 (예: "anime/protagonist_1.svg")
    frame_duration: f32,      // 각 프레임의 지속 시간
    initial_state: AnimationPlaybackState, // 초기 재생 상태
    position: Vec3,           // 엔티티의 초기 위치
    scale: Vec3,              // 엔티티의 초기 스케일
) {
    let frames: Vec<Handle<Svg>> = frame_paths
        .iter()
        .map(|path| asset_server.load(path))
        .collect();

    // 프레임이 없으면 생성하지 않음
    if frames.is_empty() {
        warn!("No animation frames provided for spawn_animated_svg.");
        return;
    }

    commands.spawn((
        Svg2d(frames[0].clone()),
        Animation {
            frames,
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            current_frame_index: 0,
            state: initial_state,
        },
        Transform::from_translation(position).with_scale(scale),
    ));
}

// 애니메이션 프레임을 업데이트하는 시스템
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Svg2d)>,
) {
    for (mut animation, mut svg2d) in query.iter_mut() {
        // 애니메이션이 재생 중일 때만 업데이트
        if animation.state == AnimationPlaybackState::Playing {
            animation.timer.tick(time.delta());

            // 타이머가 방금 완료되었는지 확인
            if animation.timer.just_finished() {
                animation.current_frame_index = (animation.current_frame_index + 1) % animation.frames.len(); // 다음 프레임 인덱스 계산 (반복)
                svg2d.0 = animation.frames[animation.current_frame_index].clone(); // .svg 대신 .0 필드에 접근
            }
        }
    }
}

// 애니메이션 시스템을 Bevy 앱에 추가하기 위한 플러그인
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}