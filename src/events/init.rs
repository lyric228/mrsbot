use crate::types::State;

pub fn init_handler(mut state: State) {
    state.flags.init = true;
}
