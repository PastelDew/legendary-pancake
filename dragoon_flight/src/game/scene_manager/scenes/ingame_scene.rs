use super::super::scene_states::SceneStatus;
use super::super::scene_traits::*;
use crate::game::entity::{
    anime::{self, *},
    entity_properties::{self, AutoSizeCollider, Collider, CollisionCheck, Velocity2D},
    spawner::*,
};
use bevy::{ecs::schedule::*, prelude::Or, prelude::*};

#[derive(Component)]
struct OnInGameScreen;

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
struct HealthBar {
    max_width: f32,
    height: f32,
}

#[derive(Component)]
struct Player {
    fire_timer: Timer,
}

#[derive(Component)]
struct Enemy {}

#[derive(Component)]
struct Bullet {
    life: Timer,
}

#[derive(Component)]
struct DyingFade {
    timer: Timer,
}

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
            // 애니메이션 프레임 갱신 후 콜라이더 자동 설정 적용
            anime::animate_sprite,
            entity_properties::auto_size_colliders_system,
        )
            .into_configs()
    }

    fn system_on_exit(&self) -> SystemConfigs {
        on_exit.into_configs()
    }
}

fn on_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut frame_cache: ResMut<FrameCache>,
) {
    // 인게임 카메라 생성
    commands.spawn((Camera2d, OnInGameScreen));
    // 창 크기 계산 및 플레이어 X 앵커
    let window = windows.single();
    let half_w = window.width() / 2.0;
    let player_margin = 60.0;
    let player_x = -half_w + player_margin;

    let player_frames = load_frames(
        &asset_server,
        vec![
            "anime/protagonist_1.png".to_string(),
            "anime/protagonist_2.png".to_string(),
            "anime/protagonist_3.png".to_string(),
        ],
    );

    let player_entity = spawn_entity(
        &mut commands,
        Animation {
            frames: player_frames,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            current_frame_index: 0,
            state: AnimationPlaybackState::Playing,
        },
        Vec3::new(player_x, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );
    commands.entity(player_entity)
        .insert((
            Player {
                fire_timer: Timer::from_seconds(0.125, TimerMode::Repeating),
            },
            Collider {
                offset: Vec3::ZERO,
                scale: Vec3::ONE,
            },
            AutoSizeCollider { multiplier: Vec2::new(0.5, 0.5), padding: Vec2::ZERO },
            Health { current: 3, max: 3 },
        ));

    let enemy_frames = load_frames(
        &asset_server,
        vec![
            "anime/enemy_1.png".to_string(),
            "anime/enemy_2.png".to_string(),
            "anime/enemy_3.png".to_string(),
        ],
    );
    frame_cache.map.insert("enemy".into(), enemy_frames);

    let bullet_frames = load_frames(
        &asset_server,
        vec![
            "anime/bullet_1.png".to_string(),
            "anime/bullet_2.png".to_string(),
            "anime/bullet_3.png".to_string(),
        ],
    );
    frame_cache.map.insert("bullet".into(), bullet_frames);

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
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
    let margin = 60.0;
    let anchor_x = -half_w + margin;

    if let Ok(mut tf) = q.get_single_mut() {
        let mut dir = Vec2::ZERO;
        // 좌우 이동 금지, 상하만 허용
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            dir.y += 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            dir.y -= 1.0;
        }
        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
        }
        // 속도는 컴포넌트에서 읽음
        let speed = 300.0; // 기본값 (Player 컴포넌트 값과 일치)
        // X는 왼쪽 끝에 고정
        tf.translation.x = anchor_x;
        // Y 이동 및 화면 내 클램프
        tf.translation.y += dir.y * speed * time.delta_secs();
        tf.translation.y = tf.translation.y.clamp(-half_h + margin, half_h - margin);
    }
}

// 자동 연사
fn player_auto_fire_system(
    time: Res<Time>,
    frame_cache: ResMut<FrameCache>,
    mut q_player: Query<(&Transform, &mut Player), (With<Player>, Without<DyingFade>)>,
    mut commands: Commands,
) {
    if let Ok((tf, mut player)) = q_player.get_single_mut() {
        player.fire_timer.tick(time.delta());
        if player.fire_timer.just_finished() {
            let frames = frame_cache.map.get("bullet").expect("Missing frames");
            let bullet_entity = spawn_entity(
                &mut commands,
                Animation {
                    frames: frames.clone(),
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    current_frame_index: 0,
                    state: AnimationPlaybackState::Playing,
                },
                tf.translation + Vec3::new(30.0, 0.0, 0.1),
                Vec3::ONE,
            );
            commands
                .entity(bullet_entity)
                .insert(Velocity2D { x: 600.0, y: 0.0 })
                .insert(Bullet {
                    life: Timer::from_seconds(2.0, TimerMode::Once),
                });
            info!("Bullet fired at x={:.1}, y={:.1}", tf.translation.x, tf.translation.y);
        }
    }
}

// 총알 이동 및 생명 주기 처리
fn bullet_update_system(
    time: Res<Time>,
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(Entity, &mut Transform, &Velocity2D, &mut Bullet), (With<Bullet>, Without<DyingFade>)>,
) {
    let window = windows.single();
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
    let margin = 80.0;
    for (e, mut tf, velocity, mut bullet) in &mut q {
        tf.translation.x += velocity.x * time.delta_secs();
        tf.translation.y += velocity.y * time.delta_secs();
        bullet.life.tick(time.delta());
        let x = tf.translation.x;
        let y = tf.translation.y;
        if x > half_w + margin
            || x < -half_w - margin
            || y > half_h + margin
            || y < -half_h - margin
            || bullet.life.finished()
        {
            commands.entity(e).despawn_recursive();
        }
    }
}

// 기존 on_update와 on_exit 함수는 그대로 유지
fn on_update() {}
fn on_exit(
    mut commands: Commands,
    mut q: Query<Entity, Or<(With<Bullet>, With<Player>, With<Enemy>, With<DyingFade>, With<OnInGameScreen>)>>,
) {
    for e in &mut q {
        commands.entity(e).despawn_recursive();
    }
}

// 적 이동
fn enemy_update_system(
    time: Res<Time>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(&mut Transform, &Velocity2D), With<Enemy>>,
) {
    let window = windows.single();
    let half_w = window.width() / 2.0;
    let margin = 60.0;
    for (mut tf, velocity) in &mut q {
        tf.translation.x -= velocity.x * time.delta_secs();
        // 화면 밖으로 나간 적은 별도 시스템에서 despawn
        let _ = half_w;
        let _ = margin;
    }
}

// 플레이어-적 충돌 처리: 플레이어 체력 감소 및 사망 처리
fn player_enemy_collision_system(
    mut commands: Commands,
    mut players: Query<(Entity, &Transform, &Collider, &mut Health), (With<Player>, Without<DyingFade>)>,
    enemies: Query<(Entity, &Transform, &Collider), (With<Enemy>, Without<DyingFade>)>,
) {
    if let Ok((p_ent, p_tf, p_col, mut hp)) = players.get_single_mut() {
        for (e_ent, e_tf, e_col) in &enemies {
            let p_collider = (p_tf, p_col);
            let e_collider = (e_tf, e_col);
            if p_collider.check_collision(&e_collider) {
                let before = hp.current;
                info!(
                    "Player-Enemy HIT: hp {} -> {} (pending)",
                    before,
                    before.saturating_sub(1)
                );
                // 적과 충돌 시 적 제거 (이미 제거되었을 수 있으므로 존재 확인)
                if let Some(mut ecmd) = commands.get_entity(e_ent) {
                    ecmd.despawn_recursive();
                }
                
                if hp.current > 0 {
                    hp.current -= 1;
                }
                
                // 체력이 0이 되면 페이드아웃 시작
                if hp.current == 0 {
                    if let Some(mut pcmd) = commands.get_entity(p_ent) {
                        pcmd.insert(DyingFade {
                            timer: Timer::from_seconds(0.4, TimerMode::Once),
                        });
                        info!("Player dying fade started");
                    }
                }
            }
        }
    }
}

// 체력바 업데이트(부모 PlayerHealth 기준으로 전경바 너비 조정)
fn health_bar_update_system(
    players: Query<(&Health, &Children)>,
    mut bars: Query<(&Parent, &mut Sprite, &HealthBar)>,
) {
    for (parent, mut sprite, hb) in &mut bars {
        if let Ok((ph, _)) = players.get(parent.get()) {
            let frac = if ph.max > 0 {
                ph.current as f32 / ph.max as f32
            } else {
                0.0
            };
            sprite.custom_size = Some(Vec2::new(hb.max_width * frac.clamp(0.0, 1.0), hb.height));
        }
    }
}

// 주기적으로 새로운 적 열(column)을 스폰
fn enemy_spawn_system(
    time: Res<Time>,
    spawner: Option<ResMut<EnemySpawner>>,
    mut commands: Commands,
    frame_cache: ResMut<FrameCache>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(mut spawner) = spawner else {
        return;
    };
    spawner.timer.tick(time.delta());
    if !spawner.timer.just_finished() {
        return;
    }

    let window = windows.single();
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
    let rows = (half_h * 2.0 / spawner.row_height).floor().max(1.0) as u32;
    let start_x = half_w + spawner.margin;
    let frames = frame_cache.map.get("enemy").expect("Missing frames");
    for r in 0..rows {
        let y = -half_h + spawner.row_height * (r as f32 + 0.5);
        let enemy_entity = spawn_entity(
            &mut commands,
            Animation {
                frames: frames.clone(),
                timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                current_frame_index: 0,
                state: AnimationPlaybackState::Playing,
            },
            Vec3::new(start_x, y, 0.0),
            Vec3::ONE,
        );
        commands
            .entity(enemy_entity)
            .insert((
                Velocity2D {
                    x: spawner.speed,
                    y,
                },
                Enemy {}, 
                Health { current: 5, max: 5 },
                Collider {
                    offset: Vec3::ZERO,
                    scale: Vec3::ONE,
                },
                AutoSizeCollider { multiplier: Vec2::new(0.80, 0.80), padding: Vec2::ZERO },
            ));
        info!("Enemy spawned at y={:.1}", y);
    }
}

// 왼쪽 화면 밖으로 나간 적들을 despawn
fn enemy_despawn_offscreen_system(
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q: Query<(Entity, &Transform), With<Enemy>>,
) {
    let window = windows.single();
    let half_w = window.width() / 2.0;
    let margin = 60.0;
    for (e, tf) in &q {
        if tf.translation.x < -half_w - margin {
            commands.entity(e).despawn_recursive();
            info!("Enemy despawned offscreen");
        }
    }
}

// 총알-적 충돌 처리
fn bullet_enemy_hit_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut enemies: Query<
        (Entity, &Transform, Option<&mut Health>),
        (With<Enemy>, Without<DyingFade>),
    >,
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
                if let Some(mut bcmd) = commands.get_entity(b_ent) {
                    bcmd.despawn_recursive();
                }
                if let Some(mut health) = health_opt {
                    let before = health.current;
                    if health.current > 0 { health.current -= 1; }
                    info!("Bullet-Enemy HIT: hp {} -> {}", before, health.current);
                    if health.current == 0 {
                        // 페이드아웃 시작
                        if let Some(mut ecmd) = commands.get_entity(e_ent) {
                            ecmd.insert(DyingFade { timer: Timer::from_seconds(0.4, TimerMode::Once) });
                            info!("Enemy dying fade started");
                        } else {
                            info!("Skip fade: enemy already despawned");
                        }
                    }
                }
                break;
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
                info!("Player despawned -> GameOver");
                next_state.set(super::super::scene_states::SceneStatus::GameOver);
            } else {
                info!("Enemy despawned after fade");
            }
        }
    }
}
