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
