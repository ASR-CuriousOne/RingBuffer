use std::hint::black_box;
use std::thread;
use std::time::Instant;

use crate::benchmarks::single_threaded::TestRes;
use crate::ring_buffers::ring_buffer_trait::{Consumer, Producer};
use crate::ring_buffers::spsc_ring_buffer::{RingBufferConsumer, RingBufferProducer};

pub fn bench_spsc_concurrent<P, C>(
    mut prod: P,
    mut cons: C,
    num_operations: u64,
    producer_core: core_affinity::CoreId,
    consumer_core: core_affinity::CoreId,
) -> Result<TestRes, String>
where
    P: Producer<Item = usize> + Send + 'static,
    C: Consumer<Item = usize> + Send + 'static,
{
    // WarmUp Runs
    for i in 0..10_000 {
        while prod.push(i).is_err() {}
        while cons.pop().is_none() {}
    }

    // BenchRuns
    let start = Instant::now();

    let producer_handle = thread::spawn(move || {
        core_affinity::set_for_current(producer_core);

        for i in 0..num_operations as usize {
            while prod.push(black_box(i)).is_err() {
                std::hint::spin_loop();
            }
        }
    });

    core_affinity::set_for_current(consumer_core);

    for _ in 0..num_operations as usize {
        loop {
            if let Some(val) = cons.pop() {
                black_box(val);
                break;
            }
            std::hint::spin_loop();
        }
    }

    producer_handle
        .join()
        .map_err(|_| "Producer thread panicked")?;

    let total_nanos = start.elapsed().as_nanos() as f64;
    let avg_latency = total_nanos / num_operations as f64;

    Ok(TestRes::new(avg_latency))
}

pub fn bench_spsc_concurrent_batch(
    mut producer: RingBufferProducer<usize>,
    mut consumer: RingBufferConsumer<usize>,
    num_operations: u64,
    batch_size: usize,
    producer_core: core_affinity::CoreId,
    consumer_core: core_affinity::CoreId,
) -> Result<TestRes, String>
where
{
    //WarmUp
    for i in 0..10_000 {
        while producer.push(i).is_err() {}
        while consumer.pop().is_none() {}
    }

    //BenchRuns
    let start = Instant::now();

    let producer_handle = thread::spawn(move || {
        core_affinity::set_for_current(producer_core);

        let mut current_item = 0;
        let total = num_operations as usize;

        while current_item < total {
            let end = (current_item + batch_size).min(total);

            let pushed = producer.push_batch(current_item..end);

            current_item += pushed;

            if pushed == 0 {
                std::hint::spin_loop();
            }
        }
    });

    core_affinity::set_for_current(consumer_core);

    let mut consumed_count = 0;
    let total = num_operations as usize;

    while consumed_count < total {
        let popped = consumer.pop_batch(batch_size, |val| {
            black_box(val);
        });

        consumed_count += popped;

        if popped == 0 {
            std::hint::spin_loop();
        }
    }

    producer_handle
        .join()
        .map_err(|_| "Producer thread panicked")?;

    let total_nanos = start.elapsed().as_nanos() as f64;
    let avg_latency = total_nanos / num_operations as f64;

    Ok(TestRes::new(avg_latency))
}


