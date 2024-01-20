use crate::types::{self, CreateSpace};

pub struct SpacesTraitTransporter {
    transport: crate::transport::Transport,
}

impl SpacesTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_spaces(&self, team_id: &str) -> Result<types::Spaces, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{}/space", team_id);
        // println!("At get_spaces");
        self.transport.get(&url)
    }

    pub fn create_space(
        &self,
        team_id: &str,
        space: CreateSpace,
    ) -> Result<types::Space, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{}/space", team_id);
        let request_body = serde_json::to_string(&space)?;
        self.transport.post(&url, request_body)
    }

    pub fn update_space(
        &self,
        space_id: &str,
        name: &str,
    ) -> Result<types::Spaces, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_space(
        &self,
        space_id: &str,
    ) -> Result<types::Spaces, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn get_space(&self, space_id: &str) -> Result<types::Spaces, Box<dyn std::error::Error>> {
        todo!()
    }
}
