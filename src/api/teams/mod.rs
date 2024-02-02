use crate::transport;
use crate::types::{CreateTeam, Team, Teams};
/// Teams Trait for the ClickUp API.
pub struct TeamsTraitTransporter {
    transport: crate::transport::Transport,
}

impl TeamsTraitTransporter {
    /// Create a new instance of the Teams Trait Transporter.
    pub fn new(transport: &crate::transport::Transport) -> TeamsTraitTransporter {
        TeamsTraitTransporter {
            transport: transport.clone(),
        }
    }

    /// Get all teams.
    pub fn get_teams(&self) -> Result<Teams, transport::Error> {
        let url = "https://api.clickup.com/api/v2/team";
        self.transport.get(url)
    }

    /// Get a specific team.
    pub fn create_team(
        &self,
        team_id: &str,
        name: &str,
        members: Vec<String>,
    ) -> Result<Team, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/group");
        let team = CreateTeam {
            name: name.to_string(),
            members,
        };
        let request_body = serde_json::to_string(&team)?;
        self.transport.post(&url, request_body)
    }

    /// Delete a specific team.
    pub fn delete_team(&self, team_id: &str) -> Result<Teams, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}");
        self.transport.delete(&url)
    }
}
