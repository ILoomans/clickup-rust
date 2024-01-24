use crate::types::{self, CreateChatViewComment, CreateComment, UpdateComment};

/// Comments Trait for the ClickUp API.
pub struct CommentsTraitTransporter {
    transport: crate::transport::Transport,
}

impl CommentsTraitTransporter {
    /// Create a new instance of the Comments Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all comments for a given task.
    pub fn get_task_comments(
        &self,
        task_id: &str,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}/comment");
        self.transport.get(&url)
    }

    /// Get all comments for a given chat view.
    pub fn get_chat_view_comments(
        &self,
        view_id: &str,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/view/{view_id}/comment");
        self.transport.get(&url)
    }

    /// Get all comments for a given list.
    pub fn get_list_comments(
        &self,
        list_id: u64,
    ) -> Result<types::Comments, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/comment");
        self.transport.get(&url)
    }

    /// Create a comment for a given task.
    pub fn create_task_comment(
        &self,
        task_id: &str,
        custom_tasks_ids: bool,
        team_id: u32,
        comment: CreateComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.clickup.com/api/v2/task/{task_id}/comment?custom_task_ids={custom_tasks_ids}&team_id={team_id}",
        );
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    /// Create a comment for a given chat view.
    pub fn create_chat_view_comment(
        &self,
        view_id: &str,
        comment: CreateChatViewComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/view/{view_id}/comment");
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    /// Create a comment for a given list.
    pub fn create_list_comment(
        &self,
        list_id: u64,
        comment: CreateComment,
    ) -> Result<types::CreateCommentResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/comment");
        let request_body = serde_json::to_string(&comment)?;
        self.transport.post(&url, request_body)
    }

    /// Update a specific comment.
    pub fn update_comment(
        &self,
        comment_id: u64,
        comment: UpdateComment,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/comment/{comment_id}");
        let request_body = serde_json::to_string(&comment)?;
        self.transport.put(&url, request_body)
    }

    /// Delete a specific comment.
    pub fn delete_comment(
        &self,
        comment_id: u64,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/comment/{comment_id}");
        self.transport.delete(&url)
    }
}
