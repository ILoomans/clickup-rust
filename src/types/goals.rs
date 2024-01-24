use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGoal {
    pub name: String,
    pub due_date: u64,
    pub description: String,
    pub multiple_owners: bool,
    pub owners: Vec<u64>,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    pub id: u64,
    pub username: String,
    pub initials: String,
    pub email: String,
    pub color: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Goal {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub date_created: String,
    pub start_date: Option<String>,
    pub due_date: String,
    pub description: String,
    pub private: bool,
    pub archived: bool,
    pub creator: u64,
    pub color: String,
    pub pretty_id: String,
    pub multiple_owners: bool,
    pub folder_id: Option<String>,
    pub members: Vec<u64>,
    pub owners: Vec<Owner>,
    // pub key_results: Vec<String>,
    pub percent_completed: u64,
    pub history: Option<Vec<String>>,
    pub pretty_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalContainer {
    pub goal: Goal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Goals {
    pub goals: Vec<Goal>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGoal {
    pub name: String,
    pub due_date: u64,
    pub description: String,
    pub rem_owners: Vec<u64>,
    pub add_owners: Vec<u64>,
    pub color: String,
}
