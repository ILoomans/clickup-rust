use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub color: Option<String>,
    pub initials: String,
    pub profilePicture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentText {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub reaction: String,
    pub date: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub comment: Vec<CommentText>,
    pub comment_text: String,
    pub user: User,
    pub reactions: Vec<Reaction>,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub comment_text: String,
    pub assignee: u32,
    pub notify_all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    pub id: u128,
    pub hist_id: String,
    pub date: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatViewComment {
    pub comment_text: String,
    pub notify_all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComment {
    pub comment_text: String,
    pub resolved: bool,
    pub notify_all: bool,
}
