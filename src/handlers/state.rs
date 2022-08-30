use std::collections::VecDeque;

use tui::widgets::TableState;

pub struct TableEvents {
    pub posts: VecDeque<String>,
    pub state: TableState,
}

impl TableEvents {
    pub fn new(posts: VecDeque<String>) -> Self {
        Self {
            posts,
            state: TableState::default(),
        }
    }

    pub fn set_items(&mut self, posts: VecDeque<String>) {
        self.posts = posts;

        self.state = TableState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.posts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.posts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
