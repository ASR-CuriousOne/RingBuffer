use core_affinity::CoreId;

use crate::ring_buffers::ring_buffer_trait::{Consumer, Producer};
use crate::ring_buffers::spsc_ring_buffer::create_queue;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

pub struct TestRes {
    pub total_items_consumed: u64,
    pub throughput: f64,
}

pub fn measure_spsc_throughput(
    capacity: usize,
    test_duration_secs: u64,
    producer_core: CoreId,
    consumer_core: CoreId,
) -> Result<TestRes, String> {
    let (mut prod, mut cons) = create_queue::<usize>(capacity);

    let keep_running = Arc::new(AtomicBool::new(true));

    let keep_running_prod = Arc::clone(&keep_running);
    let producer_handle = thread::spawn(move || {
        core_affinity::set_for_current(producer_core);
        let mut item = 0;
        while keep_running_prod.load(Ordering::Relaxed) {
            if prod.push(item).is_ok() {
                item += 1;
            } else {
                std::hint::spin_loop();
            }
        }
    });

    let keep_running_cons = Arc::clone(&keep_running);
    let consumer_handle = thread::spawn(move || {
        core_affinity::set_for_current(consumer_core);
        let mut total_consumed: u64 = 0;
        while keep_running_cons.load(Ordering::Relaxed) {
            if cons.pop().is_some() {
                total_consumed += 1;
            } else {
                std::hint::spin_loop();
            }
        }
        total_consumed
    });

    let start_time = Instant::now();
    thread::sleep(Duration::from_secs(test_duration_secs));

    keep_running.store(false, Ordering::Relaxed);
    let _ = producer_handle.join().unwrap();
    let total_items_consumed = consumer_handle.join().unwrap();
    let elapsed = start_time.elapsed().as_secs_f64();

    let throughput = (total_items_consumed as f64) / elapsed;

    Ok(TestRes {
        total_items_consumed,
        throughput,
    })
}
