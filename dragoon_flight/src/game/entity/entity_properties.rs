use bevy::prelude::*;
use bevy::math::bounding::*;

#[derive(Component)]
pub struct Velocity2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Acceleration2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Collider {
    pub offset: Vec3,
    pub scale: Vec3,
}

// 스프라이트 크기에 맞춰 콜라이더를 자동 설정하기 위한 마커/설정 컴포넌트
#[derive(Component)]
pub struct AutoSizeCollider {
    pub padding: Vec2,    // 반지름(half-extents)에 더/빼줄 여백. 기본 0.
    pub multiplier: Vec2, // 스프라이트 기반 반지름에 곱할 배율. 기본 1.
}

impl Default for AutoSizeCollider {
    fn default() -> Self {
        Self {
            padding: Vec2::ZERO,
            multiplier: Vec2::ONE, // 기본은 1배 (타이트 핏)
        }
    }
}

pub trait CollisionCheck {
    fn check_collision(&self, other: &Self) -> bool;
}

impl CollisionCheck for (&Transform, &Collider) {
    fn check_collision(&self, other: &Self) -> bool {
        let (transform_a, collider_a) = self;
        let (transform_b, collider_b) = other;

        // Apply per-entity scale to collider offset and half-extents
        let scale_a = transform_a.scale.truncate().abs();
        let scale_b = transform_b.scale.truncate().abs();

        let center_a = transform_a.translation.truncate()
            + collider_a.offset.truncate() * scale_a;
        let center_b = transform_b.translation.truncate()
            + collider_b.offset.truncate() * scale_b;

        // Treat Collider.scale as half-extents in local space
        let half_extents_a = collider_a.scale.truncate().abs() * scale_a;
        let half_extents_b = collider_b.scale.truncate().abs() * scale_b;

        let aabb1 = Aabb2d::new(center_a, half_extents_a);
        let aabb2 = Aabb2d::new(center_b, half_extents_b);

        let res = aabb1.intersects(&aabb2);
        if res {
            info!("AABB hit: a1={:?}, a2={:?}", aabb1, aabb2);
        }
        res
    }
}

// TODO(hjsong): 매번 업데이트 시 갱신이라 리소스 로드 시 혹은 에니메이션 프레임 단위로 갱신하는게 좋을 듯
pub fn auto_size_colliders_system(
    images: Res<Assets<Image>>,
    mut q: Query<(Entity, &Sprite, &mut Collider, Option<&AutoSizeCollider>)>,
) {
    for (entity, sprite, mut collider, cfg) in &mut q {
        let Some(size) = sprite.custom_size.or_else(|| {
            images
                .get(&sprite.image)
                .map(|img| {
                    // Image의 픽셀 크기 사용
                    let sz = img.size();
                    Vec2::new(sz.x as f32, sz.y as f32)
                })
        }) else {
            // 아직 스프라이트가 로드되지 않았을 수 있음
            continue;
        };

        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        let mut half = size * 0.5;
        if let Some(cfg) = cfg {
            half = half * cfg.multiplier + cfg.padding;
        }

        let new_scale = Vec3::new(half.x.abs(), half.y.abs(), collider.scale.z);
        // 변경이 있는 경우에만 갱신
        if (new_scale.truncate() - collider.scale.truncate()).length_squared() > f32::EPSILON {
            collider.scale = new_scale;
            info!(
                "Auto collider set: e={:?}, half=({:.1},{:.1})",
                entity,
                half.x,
                half.y
            );
        }
    }
}
