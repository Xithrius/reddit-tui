#[allow(dead_code)]
pub enum State {
    Normal,
    Insert,
    Help,
}

pub struct App {
    /// Which window the terminal is currently showing
    pub state: State,
}

impl App {
    pub const fn new() -> Self {
        Self {
            state: State::Normal,
        }
    }
}
