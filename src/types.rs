use crate::cfg::RuntimeConfig;
use azalea::ecs::component::Component;

#[derive(Default, Clone, Component)]
pub struct State {
    pub runtime_config: RuntimeConfig,
    // Здесь можно добавить другие поля состояния, если они понадобятся
}