use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub status: String,
    // #[serde(rename = "type")]
    // pub status_type: String,
    // pub orderindex: u32,
    // pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    pub color: String,
    pub id: String,
    pub orderindex: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub color: String,
    pub profilePicture: Option<String>,
    pub initials: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEstimates {
    pub enabled: bool,
    pub rollup: bool,
    pub per_assignee: bool,
}

// TODO: Feature is too generic
#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    pub due_dates: Option<Feature>,
    pub sprints: Option<Feature>,
    pub time_tracking: Option<Feature>,
    pub points: Option<Feature>,
    pub custom_items: Option<Feature>,
    pub priorities: Option<Feature>,
    pub tags: Option<Feature>,
    pub check_unresolved: Option<Feature>,
    pub zoom: Option<Feature>,
    pub milestones: Option<Feature>,
    pub custom_fields: Option<Feature>,
    pub dependency_warning: Option<Feature>,
    pub status_pies: Option<Feature>,
    pub multiple_assignees: Option<Feature>,
    pub time_estimates: Option<Feature>,
    pub emails: Option<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub private: bool,
    pub avatar: Option<String>,
    pub admin_can_manage: Option<bool>,
    pub statuses: Vec<Status>,
    pub multiple_assignees: bool,
    pub features: Features,
    pub archived: bool,
    pub members: Option<Vec<Member>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spaces {
    pub spaces: Vec<Space>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpace {
    pub name: String,
    pub multiple_assignees: bool,
    // optional features
    pub features: Option<CreateFeatureSpace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDates {
    pub enabled: bool,
    pub start_date: bool,
    pub remap_due_dates: bool,
    pub remap_closed_due_date: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnabledStruct {
    pub enabled: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFeatureSpace {
    pub due_dates: Option<DueDates>,
    pub time_tracking: Option<EnabledStruct>,
    pub tags: Option<EnabledStruct>,
    pub time_estimates: Option<EnabledStruct>,
    pub checklists: Option<EnabledStruct>,
    pub custom_fields: Option<EnabledStruct>,
    pub remap_dependencies: Option<EnabledStruct>,
    pub dependency_warning: Option<EnabledStruct>,
    pub portfolios: Option<EnabledStruct>,
}
