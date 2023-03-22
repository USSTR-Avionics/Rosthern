use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct Task {
    pub priority: i8,
    function: fn(),
}

impl Task {
    pub fn new(priority: i8, function: fn()) -> Task {
        Task {
            priority,
            function,
        }
    }

    pub fn get_function(&self) -> fn() {
        self.function
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
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

