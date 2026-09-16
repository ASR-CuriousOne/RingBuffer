use ring_buffer::benchmarks::multi_threaded::{bench_spsc_concurrent, bench_spsc_concurrent_batch};
use ring_buffer::ring_buffers::spsc_ring_buffer::create_queue;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let capacity_exponent: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(16);

    let num_operations = 1 << 25;
    let capacity = 1 << capacity_exponent;
    let batch_size = 32;

    let core_ids = core_affinity::get_core_ids().expect("Failed to retrieve core IDs");
    assert!(core_ids.len() >= 2, "Need at least 2 CPU cores");
    let producer_core = core_ids[0];
    let consumer_core = core_ids[1];

    println!("Benchmarking SPSC Ring Buffer");
    println!("Capacity: {}, Operations: {}", capacity, num_operations);

    let (prod1, cons1) = create_queue::<usize>(capacity);
    let single_result =
        bench_spsc_concurrent(prod1, cons1, num_operations, producer_core, consumer_core).unwrap();
    println!(
        "Avg single-element latency: {:.2} ns",
        single_result.avg_latency
    );

    for i in 3..=10 {
        let batch_size = 1 << i;

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
    }
}
