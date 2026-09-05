pub trait BufferState {
    type Item;

    fn capacity(&self) -> usize;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}

pub trait LocalQueue: BufferState {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item>;
    fn pop(&mut self) -> Option<Self::Item>;
}

pub trait SharedQueue: BufferState + Send + Sync {
    fn push(&self, item: Self::Item) -> Result<(), Self::Item>;
    fn pop(&self) -> Option<Self::Item>;
}
