use bevy::{ecs::schedule::*, prelude::*, prelude::Or};
use super::super::scene_states::SceneStatus;
use super::super::scene_traits::*;
use crate::game::entity::*;

// 플레이어와 총알 컴포넌트
#[derive(Component)]
struct Player {
    speed: f32,
    fire_timer: Timer,
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    dir: Vec2,
    life: Timer,
}

#[derive(Component)]
struct Enemy {
    speed: f32,
    row: u32,
}

#[derive(Component)]
struct EnemyHealth {
    hp: u32,
}

#[derive(Component)]
struct DyingFade {
    timer: Timer,
}

#[derive(Component)]
struct PlayerHealth { current: u32, max: u32 }

#[derive(Component)]
struct HealthBar { max_width: f32, height: f32 }

// 적 스폰 설정/타이머 리소스
#[derive(Resource)]
struct EnemySpawner {
    row_height: f32,
    col_spacing: f32,
    speed: f32,
    margin: f32,
    timer: Timer,
}

pub struct InGameScene {}

impl IScene for InGameScene {
    fn state(&self) -> SceneStatus {
        SceneStatus::InGame
    }

    fn system_on_enter(&self) -> SystemConfigs {
        // 애니메이션 엔티티를 생성하는 시스템 호출
        on_start.into_configs()
    }

    fn system_on_update(&self) -> SystemConfigs {
        (
            player_move_system,
            player_auto_fire_system,
            bullet_update_system,
            enemy_update_system,
            enemy_spawn_system,
            enemy_despawn_offscreen_system,
            bullet_enemy_hit_system,
            enemy_fadeout_system,
            player_enemy_collision_system,
            health_bar_update_system,
        )
            .into_configs()
    }

    fn system_on_exit(&self) -> SystemConfigs {
        on_exit.into_configs()
    }
}

// 게임 엔티티들을 생성하는 시스템
fn on_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // 창 크기 계산 및 플레이어 X 앵커
    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let half_h = window.height() as f32 / 2.0;
    let player_margin = 60.0;
    let player_x = -half_w + player_margin;

    // 플레이어 스폰
    let player_entity = spawn_animated_sprite(
        &mut commands,
        &asset_server,
        vec![
            "anime/protagonist_1.png".to_string(),
            "anime/protagonist_2.png".to_string(),
            "anime/protagonist_3.png".to_string(),
        ],
        0.1, // 프레임 지속 시간 (초)
        AnimationPlaybackState::Playing, // 초기 재생 상태
        Vec3::new(player_x, 0.0, 0.0), // 위치
        Vec3::new(1.0, 1.0, 1.0), // 스케일
    );
    commands.entity(player_entity).insert(Player { speed: 300.0, fire_timer: Timer::from_seconds(0.15, TimerMode::Repeating) });
    // 플레이어 체력 및 체력바(자식)
    commands.entity(player_entity).insert(PlayerHealth { current: 3, max: 3 });
    // 체력바 배경
    let bar_offset = Vec3::new(0.0, 60.0, 0.2);
    let bar_width = 80.0; let bar_height = 8.0;
    commands.entity(player_entity).with_children(|p| {
        p.spawn((
            Sprite::from_color(Color::srgb(0.2, 0.2, 0.2), Vec2::new(bar_width, bar_height)),
            Transform::from_translation(bar_offset),
        ));
        // 체력바 포그라운드
        p.spawn((
            Sprite::from_color(Color::srgb(0.2, 0.9, 0.2), Vec2::new(bar_width, bar_height)),
            Transform::from_translation(bar_offset + Vec3::new(0.0, 0.0, 0.01)),
            HealthBar { max_width: bar_width, height: bar_height },
        ));
    });

    let row_h: f32 = 100.0;
    let col_spacing = 240.0;
    let speed = 360.0;
    commands.insert_resource(EnemySpawner {
        row_height: row_h,
        col_spacing,
        speed,
        margin: 60.0,
        timer: Timer::from_seconds(col_spacing / speed, TimerMode::Repeating),
    });
}

// 플레이어 이동 입력 처리
fn player_move_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let half_h = window.height() as f32 / 2.0;
    let margin = 60.0;
    let anchor_x = -half_w + margin;

    if let Ok(mut tf) = q.get_single_mut() {
        let mut dir = Vec2::ZERO;
        // 좌우 이동 금지, 상하만 허용
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) { dir.y += 1.0; }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) { dir.y -= 1.0; }
        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
        }
        // 속도는 컴포넌트에서 읽음
        let speed = 300.0; // 기본값 (Player 컴포넌트 값과 일치)
        // X는 왼쪽 끝에 고정
        tf.translation.x = anchor_x;
        // Y 이동 및 화면 내 클램프
        tf.translation.y += dir.y as f32 * speed * time.delta_secs();
        tf.translation.y = tf.translation.y.clamp(-half_h + margin, half_h - margin);
    }
}

// 자동 연사
fn player_auto_fire_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut q_player: Query<(&Transform, &mut Player), (With<Player>, Without<DyingFade>)>,
    mut commands: Commands,
) {
    if let Ok((tf, mut player)) = q_player.get_single_mut() {
        player.fire_timer.tick(time.delta());
        if player.fire_timer.just_finished() {
            let bullet_entity = spawn_animated_sprite(
                &mut commands,
                &asset_server,
                vec![
                    "anime/bullet_1.png".to_string(),
                    "anime/bullet_2.png".to_string(),
                    "anime/bullet_3.png".to_string(),
                ],
                0.05,
                AnimationPlaybackState::Playing,
                tf.translation + Vec3::new(30.0, 0.0, 0.1),
                Vec3::new(1.0, 1.0, 1.0),
            );
            commands.entity(bullet_entity).insert(Bullet {
                speed: 600.0,
                dir: Vec2::new(1.0, 0.0),
                life: Timer::from_seconds(2.0, TimerMode::Once),
            });
        }
    }
}

// 총알 이동 및 생명 주기 처리
fn bullet_update_system(
    time: Res<Time>,
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(Entity, &mut Transform, &mut Bullet)>,
) {
    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let half_h = window.height() as f32 / 2.0;
    let margin = 80.0;
    for (e, mut tf, mut bullet) in &mut q {
        tf.translation.x += bullet.dir.x * bullet.speed * time.delta_secs();
        tf.translation.y += bullet.dir.y * bullet.speed * time.delta_secs();
        bullet.life.tick(time.delta());
        let x = tf.translation.x;
        let y = tf.translation.y;
        if x > half_w + margin || x < -half_w - margin || y > half_h + margin || y < -half_h - margin || bullet.life.finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

// 기존 on_update와 on_exit 함수는 그대로 유지
fn on_update() {}
fn on_exit(
    mut commands: Commands,
    mut q: Query<Entity, Or<(With<Bullet>, With<Player>, With<Enemy>, With<DyingFade>)>>,
) {
    for e in &mut q {
        commands.entity(e).despawn_recursive();
    }
}

// 적 이동 및 화면 밖에서 오른쪽으로 워프
fn enemy_update_system(
    time: Res<Time>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(&mut Transform, &Enemy)>,
) {
    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let margin = 60.0;
    for (mut tf, enemy) in &mut q {
        tf.translation.x -= enemy.speed * time.delta_secs();
        // 화면 밖으로 나간 적은 별도 시스템에서 despawn
        let _ = half_w; let _ = margin;
    }
}

// 플레이어-적 충돌 처리: 플레이어 체력 감소 및 사망 처리
fn player_enemy_collision_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<super::super::scene_states::SceneStatus>>,
    mut players: Query<(Entity, &Transform, &mut PlayerHealth), Without<DyingFade>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    if let Ok((p_ent, p_tf, mut hp)) = players.get_single_mut() {
        let p_pos = p_tf.translation.truncate();
        let player_r = 28.0f32;
        let enemy_r = 28.0f32;
        for (e_ent, e_tf) in &enemies {
            let e_pos = e_tf.translation.truncate();
            if p_pos.distance_squared(e_pos) <= (player_r + enemy_r).powi(2) as f32 {
                if hp.current > 0 { hp.current -= 1; }
                // 충돌한 적은 제거하여 연속 타격 방지
                commands.entity(e_ent).despawn_recursive();
                if hp.current == 0 {
                    // 플레이어 페이드아웃 시작
                    commands.entity(p_ent).insert(DyingFade { timer: Timer::from_seconds(0.6, TimerMode::Once) });
                    // 게임오버 전환은 페이드 시스템에서 처리
                }
                break;
            }
        }
    }
}

// 체력바 업데이트(부모 PlayerHealth 기준으로 전경바 너비 조정)
fn health_bar_update_system(
    players: Query<(&PlayerHealth, &Children)>,
    mut bars: Query<(&Parent, &mut Sprite, &HealthBar)>,
) {
    for (parent, mut sprite, hb) in &mut bars {
        if let Ok((ph, _)) = players.get(parent.get()) {
            let frac = if ph.max > 0 { ph.current as f32 / ph.max as f32 } else { 0.0 };
            sprite.custom_size = Some(Vec2::new(hb.max_width * frac.clamp(0.0, 1.0), hb.height));
        }
    }
}

// 주기적으로 새로운 적 열(column)을 스폰
fn enemy_spawn_system(
    time: Res<Time>,
    spawner: Option<ResMut<EnemySpawner>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(mut spawner) = spawner else { return; };
    spawner.timer.tick(time.delta());
    if !spawner.timer.just_finished() { return; }

    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let half_h = window.height() as f32 / 2.0;
    let rows = (half_h * 2.0 / spawner.row_height).floor().max(1.0) as u32;
    let start_x = half_w + spawner.margin;
    for r in 0..rows {
        let y = -half_h + spawner.row_height * (r as f32 + 0.5);
        let enemy_entity = spawn_animated_sprite(
            &mut commands,
            &asset_server,
            vec![
                "anime/enemy_1.png".to_string(),
                "anime/enemy_2.png".to_string(),
                "anime/enemy_3.png".to_string(),
            ],
            0.15,
            AnimationPlaybackState::Playing,
            Vec3::new(start_x, y, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        );
        commands
            .entity(enemy_entity)
            .insert((Enemy { speed: spawner.speed, row: r }, EnemyHealth { hp: 5 }));
    }
}

// 왼쪽 화면 밖으로 나간 적들을 despawn
fn enemy_despawn_offscreen_system(
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q: Query<(Entity, &Transform), With<Enemy>>,
) {
    let window = windows.single();
    let half_w = window.width() as f32 / 2.0;
    let margin = 60.0;
    for (e, tf) in &q {
        if tf.translation.x < -half_w - margin {
            commands.entity(e).despawn_recursive();
        }
    }
}

// 총알-적 충돌 처리: 적은 5히트 시 페이드아웃 후 디스폰
fn bullet_enemy_hit_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut enemies: Query<(Entity, &Transform, Option<&mut EnemyHealth>), (With<Enemy>, Without<DyingFade>)>,
) {
    // 단순 근접 판정 (원 충돌)
    let bullet_r = 12.0f32;
    let enemy_r = 28.0f32;

    for (b_ent, b_tf) in &mut bullets {
        let b_pos = b_tf.translation.truncate();
        for (e_ent, e_tf, health_opt) in &mut enemies {
            let e_pos = e_tf.translation.truncate();
            let dist2 = b_pos.distance_squared(e_pos);
            if dist2 <= (bullet_r + enemy_r) * (bullet_r + enemy_r) {
                // 명중: 총알 제거
                commands.entity(b_ent).despawn_recursive();
                // 체력 감소
                if let Some(mut health) = health_opt {
                    if health.hp > 0 {
                        health.hp -= 1;
                    }
                    if health.hp == 0 {
                        // 페이드아웃 시작
                        commands
                            .entity(e_ent)
                            .insert(DyingFade { timer: Timer::from_seconds(0.4, TimerMode::Once) });
                    }
                }
                break; // 한 총알은 하나만 타격
            }
        }
    }
}

// 페이드아웃 진행
fn enemy_fadeout_system(
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<super::super::scene_states::SceneStatus>>,
    mut q: Query<(Entity, &mut DyingFade, &mut Sprite, Option<&Player>)>,
) {
    for (e, mut fading, mut sprite, is_player) in &mut q {
        fading.timer.tick(time.delta());
        let total = fading.timer.duration().as_secs_f32().max(0.0001);
        let elapsed = fading.timer.elapsed().as_secs_f32().min(total);
        let alpha = 1.0 - (elapsed / total);
        sprite.color.set_alpha(alpha);
        if fading.timer.finished() {
            commands.entity(e).despawn_recursive();
            if is_player.is_some() {
                next_state.set(super::super::scene_states::SceneStatus::GameOver);
            }
        }
    }
}
