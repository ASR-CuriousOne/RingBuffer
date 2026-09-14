use super::ring_buffer_trait::{BufferState, LocalQueue};

pub struct BipBufferQueue<T> {
    data: Vec<Option<T>>,
    capacity: usize,

    a_start: usize,
    a_end: usize,

    b_end: usize,
}

impl<T> BipBufferQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);

        Self {
            data,
            capacity,
            a_start: 0,
            a_end: 0,
            b_end: 0,
        }
    }

    pub fn read_slice(&self) -> &[Option<T>] {
        if self.a_start < self.a_end {
            &self.data[self.a_start..self.a_end]
        } else {
            &[]
        }
    }
}

impl<T> BufferState for BipBufferQueue<T> {
    type Item = T;

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        (self.a_end - self.a_start) + self.b_end
    }
}

impl<T> LocalQueue for BipBufferQueue<T> {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item> {
        if self.is_full() {
            return Err(item);
        }

        if self.b_end > 0 {
            self.data[self.b_end] = Some(item);
            self.b_end += 1;
        } else if self.a_end < self.capacity {
            self.data[self.a_end] = Some(item);
            self.a_end += 1;
        } else {
            self.data[0] = Some(item);
            self.b_end = 1;
        }

        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        if self.a_start == self.a_end {
            self.a_start = 0;
            self.a_end = self.b_end;
            self.b_end = 0;
        }

        let item = self.data[self.a_start].take().unwrap();
        self.a_start += 1;

        Some(item)
    }
}
