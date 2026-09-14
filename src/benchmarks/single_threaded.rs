use std::hint::black_box;
use std::time::Instant;

use crate::ring_buffers::ring_buffer_trait::{BufferState, LocalQueue};

const MAX_BENCH_BATCH: usize = 1 << 20;

pub struct TestRes {
    pub avg_latency: f64,
}

impl TestRes {
    pub fn new(avg_latency: f64) -> Self {
        Self { avg_latency }
    }
}

pub fn push_pop<R>(rb: &mut R, num_of_push_pops: u64, iterations: u64) -> Result<TestRes, String>
where
    R: LocalQueue + BufferState<Item = usize>,
{
    let fill_target = rb.capacity().min(MAX_BENCH_BATCH) / 2;

    // Fill till half capactiy

    for i in 0..fill_target {
        let _ = rb.push(i);
    }

    // WarmUp Runs

    for _i in 0..20 {
        for j in 0..num_of_push_pops as usize {
            let _ = rb.push(j);
            let _ = rb.pop();
        }
    }

    // BenchRuns

    let start = Instant::now();

    for _i in 0..iterations {
        for j in 0..num_of_push_pops as usize {
            let _ = black_box(rb.push(black_box(j)));

            black_box(rb.pop());
        }
    }

    let total_nanos = start.elapsed().as_nanos() as f64;
    let total_operations = (iterations * num_of_push_pops) as f64;

    let true_avg_latency = total_nanos / (total_operations * 2.0);

    Ok(TestRes::new(true_avg_latency))
}

pub fn push<R>(rb: &mut R, iterations: u64) -> Result<TestRes, String>
where
    R: LocalQueue + BufferState<Item = usize>,
{
    let batch_size = rb.capacity().min(MAX_BENCH_BATCH);

    let mut total_push_nanos = 0;

    // WarmUp Runs

    for _ in 0..20 {
        for j in 0..batch_size {
            let _ = black_box(rb.push(black_box(j)));
        }
        for _ in 0..batch_size {
            black_box(rb.pop());
        }
    }

    // BenchRuns

    for _ in 0..iterations {
        let start_push = Instant::now();
        for j in 0..batch_size {
            let _ = black_box(rb.push(black_box(j)));
        }
        total_push_nanos += start_push.elapsed().as_nanos();

        for _ in 0..batch_size {
            black_box(rb.pop());
        }
    }

    let total_ops = (iterations * batch_size as u64) as f64;
    let avg_push_latency = total_push_nanos as f64 / total_ops;

    Ok(TestRes::new(avg_push_latency))
}

pub fn pop<R>(rb: &mut R, iterations: u64) -> Result<TestRes, String>
where
    R: LocalQueue + BufferState<Item = usize>,
{
    let batch_size = rb.capacity().min(MAX_BENCH_BATCH);

    let mut total_pop_nanos = 0;

    // WarmUp Runs

    for _ in 0..20 {
        for j in 0..batch_size {
            let _ = black_box(rb.push(black_box(j)));
        }
        for _ in 0..batch_size {
            black_box(rb.pop());
        }
    }

    // BenchRuns

    for _ in 0..iterations {
        for j in 0..batch_size {
            let _ = black_box(rb.push(black_box(j)));
        }

        let start_push = Instant::now();
        for _ in 0..batch_size {
            black_box(rb.pop());
        }
        total_pop_nanos += start_push.elapsed().as_nanos();
    }

    let total_ops = (iterations * batch_size as u64) as f64;
    let avg_pop_latency = total_pop_nanos as f64 / total_ops;

    Ok(TestRes::new(avg_pop_latency))
}
