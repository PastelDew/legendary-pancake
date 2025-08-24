use bevy::prelude::*;

#[derive(Component)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

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
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Arm {
    pub damage: u32,
}