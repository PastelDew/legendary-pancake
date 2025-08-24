use bevy::prelude::*;

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
