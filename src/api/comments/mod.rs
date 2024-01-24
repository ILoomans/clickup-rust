use crate::types::{self, CreateChatViewComment, CreateComment, UpdateComment};

pub struct CommentsTraitTransporter {
    transport: crate::transport::Transport,
}

impl CommentsTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_task_comments(
        &self,
        task_id: &str,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{}/comment", task_id);
        self.transport.get(&url)
    }

    pub fn get_chat_view_comments(
        &self,
        view_id: &str,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/view/{}/comment", view_id);
        self.transport.get(&url)
    }

    pub fn get_list_comments(
        &self,
        list_id: u64,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/comment", list_id);
        self.transport.get(&url)
    }

    pub fn create_task_comment(
        &self,
        task_id: &str,
        custom_tasks_ids: bool,
        team_id: u32,
        comment: CreateComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.clickup.com/api/v2/task/{}/comment?custom_task_ids={}&team_id={}",
            task_id, custom_tasks_ids, team_id
        );
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    pub fn create_chat_view_comment(
        &self,
        view_id: &str,
        comment: CreateChatViewComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/view/{}/comment", view_id);
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    pub fn create_list_comment(
        &self,
        list_id: u64,
        comment: CreateComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/comment", list_id);
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    pub fn update_comment(
        &self,
        comment_id: u64,
        comment: UpdateComment,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/comment/{}", comment_id);
        let request_body = serde_json::to_string(&comment)?;
        self.transport.put(&url, request_body)
    }

    pub fn delete_comment(
        &self,
        comment_id: u64,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/comment/{}", comment_id);
        self.transport.delete(&url)
    }
}
