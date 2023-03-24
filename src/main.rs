use rtos::priority_queue;
use rtos::tasks;

fn foo() {
    println!("foo");
}

fn bar() {
    println!("bar");
}

fn nvr() {
    println!("nvr");
}

fn rcr() {
    println!("rcr");
}

fn main() {
    println!("Hello, world!");

    let foo_task = tasks::Task::new(1, foo);
    let bar_task = tasks::Task::new(2, bar);
    let nvr_task = tasks::Task::new(-1, nvr);
    let rcr_task = tasks::Task::new(3, rcr);

    let mut pq = priority_queue::PriorityQueue::new();
    let mut rq = priority_queue::PriorityQueue::new_recurring();

    pq.add_to_queue(foo_task);
    pq.add_to_queue(bar_task);
    pq.add_to_queue(nvr_task);

    rq.add_recurring_task(rcr_task);
    
    loop {
        match pq.execute() {
            Ok(task) => println!("Task {} executed", task.get_priority()),
            Err(e) => {}
            }

        pq.sanitize();

        rq.execute();
        }

}
