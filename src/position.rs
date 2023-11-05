#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn next_char(&mut self) {
        self.x += 1;
    }

    pub fn next_line(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    pub fn previous_char(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    pub fn as_previous(self) -> Self {
        let mut copy = self;
        copy.previous_char();
        copy
    }
}
