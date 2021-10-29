pub enum State {
    Normal,
    KeybindHelp,
    Input,
}

pub struct App {
    /// Which window the terminal is currently showing
    pub state: State,
}

impl App {
    pub fn new() -> App {
        App {
            state: State::Normal,
        }
    }
}
