use ring_buffer::benchmarks::multi_threaded::{
    Event, bench_spsc_concurrent, bench_spsc_concurrent_batch,bench_spsc_latency_histogram
};
use ring_buffer::benchmarks::throughput::measure_spsc_throughput;
use ring_buffer::ring_buffers::spsc_ring_buffer::create_queue;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let capacity_exponent: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(16);

    let num_operations = 1 << 25;
    let capacity = 1 << capacity_exponent;
    let batch_size = 64;

    let core_ids = core_affinity::get_core_ids().expect("Failed to retrieve core IDs");
    assert!(core_ids.len() >= 2, "Need at least 2 CPU cores");
    let producer_core = core_ids[0];
    let consumer_core = core_ids[8];

    println!("Benchmarking SPSC Ring Buffer");
    println!("Capacity: {}, Operations: {}", capacity, num_operations);

    let (prod1, cons1) = create_queue::<usize>(capacity);
    let single_result =
        bench_spsc_concurrent(prod1, cons1, num_operations, producer_core, consumer_core).unwrap();
    println!(
        "Avg single-element latency: {:.2} ns",
        single_result.avg_latency
    );

    let (prod2, cons2) = create_queue::<usize>(capacity);
    let batch_result = bench_spsc_concurrent_batch(
        prod2,
        cons2,
        num_operations,
        batch_size,
        producer_core,
        consumer_core,
    )
    .unwrap();
    println!(
        "Avg batched latency (size {}):  {:.2} ns",
        batch_size, batch_result.avg_latency
    );

    let _ = measure_spsc_throughput(capacity, 1, producer_core, consumer_core).unwrap();
    let throughput_result =
        measure_spsc_throughput(capacity, 5, producer_core, consumer_core).unwrap();

    println!("Total Consumed: {}", throughput_result.total_items_consumed);
    println!("Throughput: {:.2} ops/sec", throughput_result.throughput);

    let (prod3, cons3) = create_queue::<Event>(capacity);
    println!("Running Latency Histogram Benchmark...");
    let histogram =
        bench_spsc_latency_histogram(prod3, cons3, num_operations, producer_core, consumer_core)
            .unwrap();

    println!("--- Latency Percentiles (Nanoseconds) ---");
    println!("Min:    {} ns", histogram.min());
    println!("p50:    {} ns", histogram.value_at_quantile(0.50));
    println!("p90:    {} ns", histogram.value_at_quantile(0.90));
    println!("p99:    {} ns", histogram.value_at_quantile(0.99));
    println!("p99.9:  {} ns", histogram.value_at_quantile(0.999));
    println!("p99.99: {} ns", histogram.value_at_quantile(0.9999));
    println!("Max:    {} ns", histogram.max());
}
