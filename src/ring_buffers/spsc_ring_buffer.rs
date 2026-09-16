use crate::ring_buffers::ring_buffer_trait::Consumer;
use crate::ring_buffers::ring_buffer_trait::Producer;

use super::ring_buffer_trait::BufferState;
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[repr(align(32))]
struct CacheAligned<T>(T);

struct RingBufferInner<T> {
    data: Box<[UnsafeCell<Option<T>>]>,
    capacity: usize,

    head: CacheAligned<AtomicUsize>,
    tail: CacheAligned<AtomicUsize>,
}

unsafe impl<T: Send> Sync for RingBufferInner<T> {}
unsafe impl<T: Send> Send for RingBufferInner<T> {}

pub struct RingBufferProducer<T> {
    inner: Arc<RingBufferInner<T>>,
}

pub struct RingBufferConsumer<T> {
    inner: Arc<RingBufferInner<T>>,
}

impl<T> BufferState for RingBufferConsumer<T> {
    type Item = T;

    fn capacity(&self) -> usize {
        self.inner.capacity
    }

    fn len(&self) -> usize {
        let curr_head = self.inner.head.0.load(Ordering::Relaxed);
        let curr_tail = self.inner.tail.0.load(Ordering::Relaxed);

        curr_head.wrapping_sub(curr_tail)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}

impl<T> BufferState for RingBufferProducer<T> {
    type Item = T;

    fn capacity(&self) -> usize {
        self.inner.capacity
    }

    fn len(&self) -> usize {
        let curr_head = self.inner.head.0.load(Ordering::Relaxed);
        let curr_tail = self.inner.tail.0.load(Ordering::Relaxed);

        curr_head.wrapping_sub(curr_tail)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}

impl<T> Producer for RingBufferProducer<T> {
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Item> {
        if self.is_full() {
            return Err(item);
        }

        let curr_head = self.inner.head.0.load(Ordering::Relaxed);

        let index = curr_head & (self.capacity() - 1);

        unsafe {
            *self.inner.data[index].get() = Some(item);
        }

        self.inner
            .head
            .0
            .store(curr_head.wrapping_add(1), Ordering::Release);
        Ok(())
    }
}

impl<T> RingBufferProducer<T> {
    pub fn push_batch<I>(&mut self, items: I) -> usize
    where
        I: IntoIterator<Item = T>,
    {
        let curr_head = self.inner.head.0.load(Ordering::Relaxed);
        let curr_tail = self.inner.tail.0.load(Ordering::Acquire);

        let capacity = self.capacity();
        let available = capacity - curr_head.wrapping_sub(curr_tail);
        if available == 0 {
            return 0;
        }

        let mut count = 0;
        let mut iter = items.into_iter();

        for _i in 0..available {
            if let Some(item) = iter.next() {
                let index = curr_head.wrapping_add(count) & (self.capacity() - 1);

                unsafe {
                    *self.inner.data[index].get() = Some(item);
                }

                count += 1;
            } else {
                break;
            }
        }

        if count > 0 {
            self.inner
                .head
                .0
                .store(curr_head.wrapping_add(count), Ordering::Release);
        }

        count
    }
}

impl<T> Consumer for RingBufferConsumer<T> {
    fn pop(&mut self) -> Option<Self::Item> {
        let curr_tail = self.inner.tail.0.load(Ordering::Relaxed);
        let curr_head = self.inner.head.0.load(Ordering::Acquire);

        if curr_head == curr_tail {
            return None;
        }

        let index = curr_tail & (self.capacity() - 1);

        let item = unsafe { (*self.inner.data[index].get()).take().unwrap() };

        self.inner
            .tail
            .0
            .store(curr_tail.wrapping_add(1), Ordering::Release);
        Some(item)
    }
}

impl<T> RingBufferConsumer<T> {
    pub fn pop_batch<F>(&mut self, max_items: usize, mut f: F) -> usize
    where
        F: FnMut(T),
    {
        let curr_tail = self.inner.tail.0.load(Ordering::Relaxed);
        let curr_head = self.inner.head.0.load(Ordering::Acquire);

        let available = curr_head.wrapping_sub(curr_tail);
        let to_read = available.min(max_items);

        if to_read == 0 {
            return 0;
        }

        for i in 0..to_read {
            let index = curr_tail.wrapping_add(i) & (self.capacity() - 1);
            let item = unsafe { (*self.inner.data[index].get()).take().unwrap() };
            f(item);
        }

        self.inner
            .tail
            .0
            .store(curr_tail.wrapping_add(to_read), Ordering::Release);

        to_read
    }
}

pub fn create_queue<T>(capacity: usize) -> (RingBufferProducer<T>, RingBufferConsumer<T>) {
    assert!(
        capacity > 0 && capacity.is_power_of_two(),
        "Capacity must be a power of two"
    );

    let mut vec = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        vec.push(UnsafeCell::new(None));
    }

    let inner = Arc::new(RingBufferInner {
        data: vec.into_boxed_slice(),
        capacity,
        head: CacheAligned(AtomicUsize::new(0)),
        tail: CacheAligned(AtomicUsize::new(0)),
    });

    (
        RingBufferProducer {
            inner: Arc::clone(&inner),
        },
        RingBufferConsumer { inner },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ring_buffers::tests;

    #[test]
    fn basic_push_pop() {
        let (prod, cons) = create_queue::<i32>(16);
        tests::verify_producer_consumer_basic_push_pop(prod, cons);
    }

    #[test]
    fn push_full() {
        let (prod, cons) = create_queue::<String>(2);
        tests::verify_producer_consumer_push_full(prod, cons);
    }

    #[test]
    fn wrap_around() {
        let (prod, cons) = create_queue::<i32>(2);
        tests::verify_producer_consumer_wrap_around(prod, cons);
    }

    #[test]
    fn concurrent_stress() {
        let (prod, cons) = create_queue::<usize>(2048);
        tests::verify_producer_consumer_concurrent(prod, cons);
    }
}
