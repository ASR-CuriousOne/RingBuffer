use super::ring_buffer_trait::{BufferState, LocalQueue};
use std::fmt::Debug;

pub fn verify_basic_push_pop<Q>(mut queue: Q)
where
    Q: LocalQueue<Item = i32> + BufferState,
{
    assert_eq!(queue.push(10), Ok(()));
    assert_eq!(queue.push(20), Ok(()));

    assert_eq!(queue.len(), 2);
    assert!(!queue.is_empty());

    assert_eq!(queue.pop(), Some(10));
    assert_eq!(queue.pop(), Some(20));

    assert!(queue.is_empty());
}

pub fn verify_push_full<Q>(mut queue: Q)
where
    Q: LocalQueue<Item = String> + BufferState,
{
    assert_eq!(queue.push("A".to_string()), Ok(()));
    assert_eq!(queue.push("B".to_string()), Ok(()));

    assert!(queue.is_full());
    assert_eq!(queue.push("C".to_string()), Err("C".to_string()));
    assert_eq!(queue.len(), 2);
}

pub fn verify_wrap_around<Q>(mut queue: Q)
where
    Q: LocalQueue<Item = i32> + BufferState,
{
    queue.push(1).unwrap();
    queue.push(2).unwrap();
    queue.push(3).unwrap();

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));

    queue.push(4).unwrap();
    queue.push(5).unwrap();

    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), Some(4));
    assert_eq!(queue.pop(), Some(5));
    assert!(queue.is_empty());
}
