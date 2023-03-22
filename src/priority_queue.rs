use std::result::Result;
use crate::tasks::Task;

pub struct PriorityQueue {
    queue: Vec<Task>,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        PriorityQueue {
            queue: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn sanitize(&mut self) {
        self.queue.retain(|task| task.get_priority() >= 0);
    }

    pub fn add_to_queue(&mut self, task: Task) {
        self.queue.push(task);
        self.queue.sort();
    }

    pub fn add_recurring_task(&mut self, mut task: Task) {
        task.set_recurring(true);
        self.add_to_queue(task);
    }

    pub fn execute(&mut self) -> Result<Task, &str> {
        if self.queue.len() == 0 {
            return Err("No tasks in queue");
        }
        let curr_task = self.queue[0].clone();

        if curr_task.get_priority() < 0 {
            return Err("Tasks with a negative priority are never executed");
        }

        if !curr_task.is_recurring() {
            self.queue.remove(0);
        }

        curr_task.get_function()();

        Ok(curr_task)
    }

}

