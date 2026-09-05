use super::ring_buffer_trait::{BufferState, LocalQueue};

struct FixedSizeHeapCircularQueue<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}

impl<T> FixedSizeHeapCircularQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");

        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);

        Self {
            data,
            head: 0,
            tail: 0,
            len: 0,
            capacity,
        }
    }
}

impl<T> BufferState for FixedSizeHeapCircularQueue<T> {
    type Item = T;

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl<T> LocalQueue for FixedSizeHeapCircularQueue<T> {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item> {
        if self.is_full() {
            return Err(item);
        }

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.len += 1;

        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let item = self.data[self.head].take().unwrap();
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ring_buffers::tests;

    #[test]
    fn basic_push_pop() {
        let queue = FixedSizeHeapCircularQueue::<i32>::new(3);
        tests::verify_basic_push_pop(queue);
    }

    #[test]
    fn push_full() {
        let queue = FixedSizeHeapCircularQueue::<String>::new(2);
        tests::verify_push_full(queue);
    }

    #[test]
    fn wrap_around() {
        let queue = FixedSizeHeapCircularQueue::<i32>::new(3);
        tests::verify_wrap_around(queue);
    }
}
