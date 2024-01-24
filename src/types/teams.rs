use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Teams {
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub color: String,
    pub avatar: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub user: MemberInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberInfo {
    pub id: i64,
    pub username: String,
    pub color: String,
    pub email: String,
    pub initials: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeam {
    pub name: String,
    pub members: Vec<String>,
}
