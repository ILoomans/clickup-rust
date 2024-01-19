use crate::types::Teams;

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

    pub fn create_team(&self, name: &str) -> Result<Teams, Box<dyn std::error::Error>> {
        todo!()
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
