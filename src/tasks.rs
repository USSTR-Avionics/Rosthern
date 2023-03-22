use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct Task {
    pub priority: i8,
    function: fn(),
    recurring: bool,
}

impl Task {
    pub fn new(priority: i8, function: fn()) -> Task {
        Task {
            priority,
            function,
            recurring: false,
        }
    }

    pub fn get_function(&self) -> fn() {
        self.function
    }

    pub fn get_priority(&self) -> i8 {
        self.priority
    }

    pub fn set_recurring(&mut self, recurring: bool) {
        self.recurring = recurring;
    }

    pub fn is_recurring(&self) -> bool {
        self.recurring
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.priority.cmp(&other.priority)
        other.priority.cmp(&self.priority)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

