use crate::types::{self, CreateTask};

/// Tasks Trait for the ClickUp API.
pub struct TasksTraitTransporter {
    transport: crate::transport::Transport,
}

impl TasksTraitTransporter {
    /// Create a new instance of the Tasks Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all tasks.
    pub fn get_tasks(&self, list_id: &str) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/task");
        self.transport.get(&url)
    }

    /// Create a specific task.
    pub fn create_task(
        &self,
        list_id: &str,
        task: CreateTask,
    ) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/task");
        let request_body = serde_json::to_string(&task)?;
        self.transport.post(&url, request_body)
    }

    /// Update a specific task.
    pub fn update_task(
        &self,
        task_id: &str,
        name: CreateTask,
    ) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}");
        let request_body = serde_json::to_string(&name)?;
        self.transport.put(&url, request_body)
    }

    /// Delete a specific task.
    pub fn delete_task(
        &self,
        task_id: &str,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}");
        self.transport.delete(&url)
    }

    /// Get a specific task.
    pub fn get_task(&self, task_id: &str) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}");
        self.transport.get(&url)
    }
}
