use ring_buffer::benchmarks::single_threaded::{pop, push, push_pop};
use ring_buffer::ring_buffers::linked_list_queue::StdLinkedListQueue;

fn main() {
    let num_of_push_pops = 100000;
    let iterations = 1000;

    let mut rb = StdLinkedListQueue::<usize>::new();

    let res_push_pop = push_pop(&mut rb, num_of_push_pops, iterations).unwrap();
    let res_push = push(&mut rb, iterations).unwrap();
    let res_pop = pop(&mut rb, iterations).unwrap();

    println!("Avg push pop latency {} ns", res_push_pop.avg_latency);
    println!("Avg push latency     {} ns", res_push.avg_latency);
    println!("Avg pop latency      {} ns", res_pop.avg_latency);
}
