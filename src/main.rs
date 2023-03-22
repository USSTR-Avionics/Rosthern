use rtos::priority_queue;
use rtos::tasks;

fn foo() {
    println!("foo");
}

fn main() {
    println!("Hello, world!");

    let foo_task = tasks::Task::new(1, foo);

    let mut pq = priority_queue::PriorityQueue::new();

    pq.add_to_queue(foo_task);

    pq.execute();
}
