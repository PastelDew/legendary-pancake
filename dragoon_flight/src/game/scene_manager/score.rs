use bevy::ecs::system::Resource;

#[derive(Resource,Default, Debug, Clone, Copy)]
pub struct Score {
    pub value: u32,
}

