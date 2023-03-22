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

    pq.add_to_queue(foo_task);
    pq.add_to_queue(bar_task);
    pq.add_to_queue(nvr_task);
    pq.add_recurring_task(rcr_task);

    match pq.execute() {
        Ok(task) => println!("Task {} executed", task.get_priority()),
        Err(e) => println!("Error: {}", e),
    }

    match pq.execute() {
        Ok(task) => println!("Task {} executed", task.get_priority()),
        Err(e) => println!("Error: {}", e),
    }

    match pq.execute() {
        Ok(task) => println!("Task {} executed", task.get_priority()),
        Err(e) => println!("Error: {}", e),
    }

    match pq.execute() {
        Ok(task) => println!("Task {} executed", task.get_priority()),
        Err(e) => println!("Error: {}", e),
    }

    pq.sanitize();

    match pq.execute() {
        Ok(task) => println!("Task {} executed", task.get_priority()),
        Err(e) => println!("Error: {}", e),
    }
}
