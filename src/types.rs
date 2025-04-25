use crate::config::RuntimeConfig;
use azalea::ecs::component::Component;

#[derive(Default, Clone, Component)]
pub struct State {
    pub config: RuntimeConfig,
    pub counters: Counters,
    // Здесь можно добавить другие поля состояния, если они понадобятся
}

#[derive(Default, Clone)]
pub struct Counters {
    pub spawn: i32, 
    // Добавьте сюда другие счетчики по мере необходимости
}
