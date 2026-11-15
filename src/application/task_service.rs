use crate::domain::task::{Priority, Status, Task};
use std::collections::HashMap;

pub struct TaskService{
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TaskService {
    pub fn new() -> Self {
        Self{
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String) -> u64{
        let task = Task{
            id: self.next_id,
            title,
            priority: Priority::Medium,
            deadline: None,
            status: Status::Todo,
        };

        let id = task.id;
        self.tasks.insert(id, task);
        self.next_id += 1;

        return id;
    }

    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
    self.tasks.values()
    }

    pub fn get_task(&self, id: u64) -> Option<&Task>{
        self.tasks.get(&id)
    }

    pub fn delete_task(&mut self, id: u64) -> Option<Task>{
        self.tasks.remove(&id)
    }

    pub fn change_status(&mut self, id: u64, status: Status) -> Option<&Task>{
        let task = self.tasks.get_mut(&id)?;
        task.status = status;

        Some(task)
    }

    pub fn change_priority(&mut self, id: u64, priority: Priority) -> Option<&Task>{
        let task = self.tasks.get_mut(&id)?;
        task.priority = priority;

        Some(task)
    }

    pub fn edit_title(&mut self, id: u64, title: String) -> Option<&Task>{
        let task = self.tasks.get_mut(&id)?;
        task.title = title;

        Some(task)
    }
}


