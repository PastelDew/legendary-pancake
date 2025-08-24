use bevy::prelude::*;

// 애니메이션 재생 상태를 정의하는 열거형
#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub enum AnimationPlaybackState {
    Playing,
    Stopped,
}

// 애니메이션의 데이터를 저장하는 컴포넌트
#[derive(Component)]
pub struct Animation {
    pub frames: Vec<Handle<Image>>, // 애니메이션 프레임 PNG(Image) 핸들 목록
    pub timer: Timer,             // Bevy의 Timer를 사용하여 프레임 전환 관리
    pub current_frame_index: usize, // 현재 표시 중인 프레임 인덱스
    pub state: AnimationPlaybackState, // 애니메이션 재생 상태
}

pub fn load_frames(
    asset_server: &Res<AssetServer>,
    frame_paths: Vec<String>
) -> Vec<Handle<Image>> {
    frame_paths
        .iter()
        .map(|path| asset_server.load(path))
        .collect()
}

// 애니메이션 엔티티를 생성하는 헬퍼 함수 (PNG 스프라이트)
pub fn spawn_animated_sprite(
    commands: &mut Commands,
    anime: Animation,
    position: Vec3,           // 엔티티의 초기 위치
    scale: Vec3,              // 엔티티의 초기 스케일
) -> Entity {
    commands.spawn((
        Sprite::from_image(anime.frames[0].clone()),
        Transform::from_translation(position).with_scale(scale),
        anime,
    ))
    .id()
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        // 애니메이션이 재생 중일 때만 업데이트
        if animation.state == AnimationPlaybackState::Playing {
            animation.timer.tick(time.delta());

            // 타이머가 방금 완료되었는지 확인
            if animation.timer.just_finished() {
                animation.current_frame_index = (animation.current_frame_index + 1) % animation.frames.len(); // 다음 프레임 인덱스 계산 (반복)
                sprite.image = animation.frames[animation.current_frame_index].clone();
            }
        }
    }
}
