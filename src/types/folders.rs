use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub orderindex: u32,
    pub color: String,
    #[serde(rename = "type")]
    pub status_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub access: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub orderindex: u32,
    // pub status: Option<Status>,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub task_count: u32,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub space: Option<Space>,
    pub archived: bool,
    pub override_statuses: Option<bool>,
    pub statuses: Vec<Status>,
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub orderindex: u32,
    pub override_statuses: bool,
    pub hidden: bool,
    pub space: Space,
    pub task_count: String,
    pub archived: bool,
    // pub statuses: Vec<Status>,
    pub lists: Vec<List>,
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folders {
    pub folders: Vec<Folder>,
}
