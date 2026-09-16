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

pub trait Producer: BufferState {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item>;
}

pub trait Consumer: BufferState {
    fn pop(&mut self) -> Option<Self::Item>;
}

pub trait SPSCQueue {
    type Item;
    type ProducerHandle: Producer<Item = Self::Item>;
    type ConsumerHandle: Consumer<Item = Self::Item>;

    fn new(self) -> (Self::ProducerHandle, Self::ConsumerHandle);
}
