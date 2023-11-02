pub struct Stream<T> {
    pub elements: Vec<T>,
    pub index: usize,
}

impl<T: Clone> Stream<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Stream { elements, index: 0 }
    }

    pub fn peek(&self) -> Option<T> {
        self.elements.get(self.index).cloned()
    }

    pub fn consume(&mut self) -> Option<T> {
        let element = self.peek();

        if self.index < self.elements.len() {
            self.index += 1;
        }

        element
    }
}

impl<T: Clone> From<Vec<T>> for Stream<T> {
    fn from(val: Vec<T>) -> Self {
        Stream::new(val)
    }
}
