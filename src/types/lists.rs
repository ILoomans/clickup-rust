use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub color: String,
    pub hide_label: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub orderindex: u32,
    pub status: Option<Status>,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub task_count: Option<u32>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub folder: Folder,
    pub space: Space,
    // pub priority: String,
    // pub assignee: String,
    // pub task_count: u32,
    // pub due_date: String,
    // pub start_date: String,
    // // pub space: Space,
    // pub archived: bool,
    // pub override_statuses: bool,
    // // pub statuses: Status,
    // pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lists {
    pub lists: Vec<List>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateList {
    pub name: String,
    pub content: Option<String>,
    pub due_date: Option<u64>,
    pub due_date_time: Option<bool>,
    pub priority: Option<u32>,
    pub assignee: Option<u64>,
    pub status: Option<String>,
}
