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
    pub fn new() -> Self {
        App {
            state: State::Normal,
        }
    }
}
