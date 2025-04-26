use crate::config::RuntimeConfig;
use azalea::{ecs::component::Component, Vec3};

#[derive(Default, Clone, Component)]
pub struct State {
    pub config: RuntimeConfig,
    pub counters: Counters,
    pub flags: Flags,
    pub prev_pos: Vec3,
}

#[derive(Default, Clone)]
pub struct Counters {
    pub spawn: i32, 
}

#[derive(Default, Clone)]
pub struct Flags {
    pub init: bool,
    pub login: bool,
}
