use crate::types::*;

pub fn spawn_handler(mut state: State) {
    state.counters.spawn += 1;
}
