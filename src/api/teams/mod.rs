use crate::types::{CreateTeam, Team, Teams};

pub struct TeamsTraitTransporter {
    transport: crate::transport::Transport,
}

impl TeamsTraitTransporter {
    pub fn new(transport: &crate::transport::Transport) -> TeamsTraitTransporter {
        TeamsTraitTransporter {
            transport: transport.clone(),
        }
    }

    pub fn get_teams(&self) -> Result<Teams, Box<dyn std::error::Error>> {
        let url = "https://api.clickup.com/api/v2/team";
        // println!("At get_teams");
        self.transport.get(url)
    }

    pub fn create_team(
        &self,
        team_id: &str,
        name: &str,
        members: Vec<String>,
    ) -> Result<Team, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{}/group", team_id);
        let team = CreateTeam {
            name: name.to_string(),
            members,
        };
        // serialize team
        // println!("At create_team");
        let request_body = serde_json::to_string(&team)?;

        self.transport.post(&url, request_body)
    }

    pub fn update_team(
        &self,
        team_id: &str,
        name: &str,
    ) -> Result<Teams, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_team(&self, team_id: &str) -> Result<Teams, Box<dyn std::error::Error>> {
        todo!()
    }
}
