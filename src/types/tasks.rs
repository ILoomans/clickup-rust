use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub color: String,
    pub r#type: String,
    pub orderindex: u32,
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
    id: String,
    custom_id: Option<String>,
    custom_item_id: u32,
    name: String,
    text_content: Option<String>,
    description: Option<String>,
    status: Status,
    orderindex: String,
    date_created: String,
    date_updated: String,
    date_closed: Option<String>,
    date_done: Option<String>,
    archived: bool,
    creator: Creator,
    assignees: Vec<Assignee>,
    // TODO: Figure out what watcher looks like
    // watchers: Vec<Watcher>,
    // checklists: Vec<Checklist>,
    // tags: Vec<Tag>,
    // parent: Option<Parent>,
    // priority: Option<Priority>,
    due_date: Option<String>,
    start_date: Option<String>,
    points: Option<u32>,
    time_estimate: Option<u32>,
    // custom_fields: Vec<CustomField>,
    // dependencies: Vec<Dependency>,
    // linked_tasks: Vec<LinkedTask>,
    // locations: Vec<Location>,
    team_id: String,
    url: String,
    sharing: Sharing,
    permission_level: String,
    list: List,
    project: Project,
    folder: Folder,
    space: Space,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}
