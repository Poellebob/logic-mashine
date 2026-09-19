pub mod editor;

use crate::core::logic;

#[derive(Default)]
pub struct State {
    gate: logic::Gate,
}

pub struct App {
    pub state: State,
}

impl App {
    pub fn init() -> Self {
        return App {
            state: State::default(),
        };
    }
}
