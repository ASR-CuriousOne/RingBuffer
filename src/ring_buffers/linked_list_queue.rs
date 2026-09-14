use super::ring_buffer_trait::{BufferState, LocalQueue};
use std::collections::LinkedList;

pub struct StdLinkedListQueue<T> {
    inner: LinkedList<T>,
}

impl<T> StdLinkedListQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }
}

impl<T> BufferState for StdLinkedListQueue<T> {
    type Item = T;

    fn capacity(&self) -> usize {
        usize::MAX
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> LocalQueue for StdLinkedListQueue<T> {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item> {
        if self.is_full() {
            Err(item)
        } else {
            self.inner.push_back(item);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            self.inner.pop_front()
        }
    }
}
