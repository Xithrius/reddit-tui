use std::collections::VecDeque;

pub struct Data {
    /// The author of the post.
    pub author: String,
    /// The permalink to the post.
    pub link: String,
    /// The content of the post.
    pub content: String,
}

pub enum State {
    Normal,
    KeybindHelp,
    Input,
}

pub struct App {
    /// All the posts that have been received from Reddit.
    pub feed: VecDeque<Data>,
    /// Which window the terminal is currently showing
    pub state: State,
}

impl App {
    pub fn new(data_limit: usize) -> App {
        App {
            feed: VecDeque::with_capacity(data_limit),
            state: State::Normal,
        }
    }
}
