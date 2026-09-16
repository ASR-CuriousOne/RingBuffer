use super::ring_buffer_trait::{BufferState, Consumer, LocalQueue, Producer};
use std::thread;

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

pub fn verify_producer_consumer_basic_push_pop<P, C>(mut prod: P, mut cons: C)
where
    P: Producer<Item = i32> + BufferState,
    C: Consumer<Item = i32> + BufferState,
{
    assert_eq!(prod.push(10), Ok(()));
    assert_eq!(prod.push(20), Ok(()));
    assert_eq!(prod.len(), 2);
    assert!(!prod.is_empty());

    assert_eq!(cons.pop(), Some(10));
    assert_eq!(cons.pop(), Some(20));
    assert!(cons.is_empty());
}

pub fn verify_producer_consumer_push_full<P, C>(mut prod: P, _cons: C)
where
    P: Producer<Item = String> + BufferState,
    C: Consumer<Item = String> + BufferState,
{
    assert_eq!(prod.push("A".to_string()), Ok(()));
    assert_eq!(prod.push("B".to_string()), Ok(()));
    assert!(prod.is_full());
    assert_eq!(prod.push("C".to_string()), Err("C".to_string()));
    assert_eq!(prod.len(), 2);
}

pub fn verify_producer_consumer_wrap_around<P, C>(mut prod: P, mut cons: C)
where
    P: Producer<Item = i32> + BufferState,
    C: Consumer<Item = i32> + BufferState,
{
    prod.push(1).unwrap();
    prod.push(2).unwrap();

    assert_eq!(cons.pop(), Some(1));
    assert_eq!(cons.pop(), Some(2));

    prod.push(4).unwrap();
    prod.push(5).unwrap();

    assert_eq!(cons.pop(), Some(4));
    assert_eq!(cons.pop(), Some(5));
    assert!(cons.is_empty());
}

pub fn verify_producer_consumer_concurrent<P, C>(mut prod: P, mut cons: C)
where
    P: Producer<Item = usize> + Send + 'static,
    C: Consumer<Item = usize> + Send + 'static,
{
    let num_iterations = 1_000_000;

    let producer_handle = thread::spawn(move || {
        for i in 0..num_iterations {
            while prod.push(i).is_err() {
                std::hint::spin_loop();
            }
        }
    });

    for i in 0..num_iterations {
        loop {
            if let Some(val) = cons.pop() {
                assert_eq!(val, i, "Popped value does not match expected sequence");
                break;
            }
            std::hint::spin_loop();
        }
    }

    producer_handle.join().expect("Producer thread panicked");
}
