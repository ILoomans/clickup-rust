use crate::types;

pub struct TasksTraitTransporter {
    transport: crate::transport::Transport,
}

impl TasksTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_tasks(&self, list_id: &str) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/task", list_id);
        // println!("At get_tasks");
        self.transport.get(&url)
    }

    pub fn create_task(
        &self,
        list_id: &str,
        name: &str,
    ) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn update_task(
        &self,
        task_id: &str,
        name: &str,
    ) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_task(&self, task_id: &str) -> Result<types::Tasks, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn get_task(&self, task_id: &str) -> Result<types::Task, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{}", task_id);
        // println!("At get_task");
        self.transport.get(&url)
    }
}
