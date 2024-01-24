use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub display_profile: Option<bool>,
    pub verified_ambassador: Option<bool>,
    pub verified_consultant: Option<bool>,
    pub top_tier_user: Option<bool>,
    pub viewed_verified_ambassador: Option<bool>,
    pub viewed_verified_consultant: Option<bool>,
    pub viewed_top_tier_user: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub color: String,
    pub initials: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    #[serde(rename = "profilePicture")]
    pub profile_info: Option<ProfileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Members {
    pub members: Vec<Member>,
}
