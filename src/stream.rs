pub struct Stream<T> {
    elements: Vec<T>,
    index: usize,

    /// Used for keeping track of the current position in the input
    /// This can be modified by the lexer to keep track of the current line and column
    pub visual_index: usize,
}

impl<T: Clone> Stream<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Stream {
            elements,
            index: 0,
            visual_index: 0,
        }
    }

    pub fn peek(&self) -> Option<T> {
        self.elements.get(self.index).cloned()
    }

    pub fn consume(&mut self) -> Option<T> {
        let element = self.peek();

        if self.index < self.elements.len() {
            self.index += 1;
            self.visual_index += 1;
        }

        element
    }

    pub fn unconsume(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.visual_index -= 1;
        }
    }
}

impl<T: Clone> From<Vec<T>> for Stream<T> {
    fn from(val: Vec<T>) -> Self {
        Stream::new(val)
    }
}
