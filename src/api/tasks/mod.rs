use crate::types::{self, CreateTask};

pub struct TasksTraitTransporter {
    transport: crate::transport::Transport,
}

impl TasksTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_tasks(&self, list_id: &str) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/task", list_id);
        self.transport.get(&url)
    }

    pub fn create_task(
        &self,
        list_id: &str,
        task: CreateTask,
    ) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/task", list_id);
        let request_body = serde_json::to_string(&task)?;
        self.transport.post(&url, request_body)
    }

    pub fn update_task(
        &self,
        task_id: &str,
        name: &str,
    ) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_task(
        &self,
        task_id: &str,
    ) -> Result<types::EmpptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{}", task_id);
        self.transport.delete(&url)
    }

    pub fn get_task(&self, task_id: &str) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{}", task_id);
        self.transport.get(&url)
    }
}
