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

    pub fn add_to_queue(&mut self, task: Task) {
        self.queue.push(task);
        self.queue.sort();
    }

    pub fn execute(&mut self) {
        let curr_task = self.queue[0].clone();
        self.queue.remove(0);
        curr_task.get_function()();
    }

}

