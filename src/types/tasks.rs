use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: Option<String>,
    pub color: String,
    pub r#type: String,
    pub orderindex: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    pub id: u64,
    pub username: String,
    pub color: String,
    pub email: String,
    pub profilePicture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignee {
    pub id: u64,
    pub username: String,
    pub color: String,
    pub initials: String,
    pub email: String,
    pub profilePicture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sharing {
    pub public: bool,
    pub public_share_expires_on: Option<String>,
    pub public_fields: Vec<String>,
    pub token: Option<String>,
    pub seo_optimized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    id: String,
    name: String,
    access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    hidden: bool,
    access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    id: String,
    name: String,
    hidden: bool,
    access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub custom_id: Option<String>,
    pub custom_item_id: u32,
    pub name: String,
    pub text_content: Option<String>,
    pub description: Option<String>,
    pub status: Status,
    pub orderindex: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_closed: Option<String>,
    pub date_done: Option<String>,
    pub archived: bool,
    pub creator: Creator,
    pub assignees: Vec<Assignee>,
    // TODO: Figure out what watcher looks like
    // watchers: Vec<Watcher>,
    // checklists: Vec<Checklist>,
    // tags: Vec<Tag>,
    // parent: Option<Parent>,
    // priority: Option<Priority>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub points: Option<u32>,
    pub time_estimate: Option<u32>,
    // custom_fields: Vec<CustomField>,
    // dependencies: Vec<Dependency>,
    // linked_tasks: Vec<LinkedTask>,
    // locations: Vec<Location>,
    pub team_id: String,
    pub url: String,
    pub sharing: Sharing,
    pub permission_level: String,
    pub list: List,
    pub project: Project,
    pub folder: Folder,
    pub space: Space,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomField {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub name: String,
    pub description: Option<String>,
    pub markdown_description: Option<String>,
    pub assignees: Option<Vec<u64>>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub priority: Option<u32>,
    pub due_date: Option<u64>,
    pub due_date_time: Option<bool>,
    pub time_estimate: Option<u64>,
    pub start_date: Option<u64>,
    pub start_date_time: Option<bool>,
    pub notify_all: Option<bool>,
    pub parent: Option<String>,
    pub links_to: Option<String>,
    pub check_required_custom_fields: Option<bool>,
    pub custom_fields: Option<Vec<CustomField>>,
}
