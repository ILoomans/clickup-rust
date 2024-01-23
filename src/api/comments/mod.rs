use crate::types;

pub struct CommentsTraitTransporter {
    transport: crate::transport::Transport,
}


impl CommentsTraitTransporter {

    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_task_comments(&self, task_id: &str) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{}/comment", task_id);
        self.transport.get(&url)
    }

    pub fn get_chat_view_comments(&self, view_id: &str) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/view/{}/comment", view_id);
        self.transport.get(&url)
    }
}