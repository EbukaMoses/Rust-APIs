struct Task {
    description: String,
    done: bool,
}
impl Task {
    // Task constructor
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }

    //adding task
    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    //mark or unmark task as done
    fn mark_task_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = !task.done;
        }
    }

    //remove task
    fn remove_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = !task.done;
        }
    }

    //show atsk
    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[]" };
            println!("{} {} : {}", index + 1, task.description, status);
        }
    }

    
}