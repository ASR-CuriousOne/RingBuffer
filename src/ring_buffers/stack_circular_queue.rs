use super::ring_buffer_trait::{BufferState, LocalQueue};

pub struct FixedSizeStackCircularQueue<T, const CAPACITY: usize> {
    data: [Option<T>; CAPACITY],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const CAPACITY: usize> FixedSizeStackCircularQueue<T, CAPACITY> {
    pub fn new() -> Self {
        const {
            assert!(CAPACITY > 0, "Capacity must be greater than 0");
        }

        Self {
            data: [const { None }; CAPACITY],
            head: 0,
            tail: 0,
            len: 0,
        }
    }
}

impl<T, const CAPACITY: usize> BufferState for FixedSizeStackCircularQueue<T, CAPACITY> {
    type Item = T;

    fn capacity(&self) -> usize {
        CAPACITY
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == CAPACITY
    }
}

impl<T, const CAPACITY: usize> LocalQueue for FixedSizeStackCircularQueue<T, CAPACITY> {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item> {
        if self.is_full() {
            return Err(item);
        }

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % CAPACITY;
        self.len += 1;

        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let item = self.data[self.head].take().unwrap();
        self.head = (self.head + 1) % CAPACITY;
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
        let queue = FixedSizeStackCircularQueue::<i32, 3>::new();
        tests::verify_basic_push_pop(queue);
    }

    #[test]
    fn push_full() {
        let queue = FixedSizeStackCircularQueue::<String, 2>::new();
        tests::verify_push_full(queue);
    }

    #[test]
    fn wrap_around() {
        let queue = FixedSizeStackCircularQueue::<i32, 3>::new();
        tests::verify_wrap_around(queue);
    }
}
