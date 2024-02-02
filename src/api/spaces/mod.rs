use crate::transport;
use crate::types::{self, CreateSpace};
/// Spaces Trait for the ClickUp API.
pub struct SpacesTraitTransporter {
    transport: crate::transport::Transport,
}

impl SpacesTraitTransporter {
    /// Create a new instance of the Spaces Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all spaces.
    pub fn get_spaces(&self, team_id: &str) -> Result<types::Spaces, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/space");
        self.transport.get(&url)
    }

    /// Create a specific space.
    pub fn create_space(
        &self,
        team_id: &str,
        space: CreateSpace,
    ) -> Result<types::Space, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/space");
        let request_body = serde_json::to_string(&space)?;
        self.transport.post(&url, request_body)
    }

    /// Update a specific space.
    pub fn update_space(
        &self,
        space_id: &str,
        space: CreateSpace,
    ) -> Result<types::Space, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/space/{space_id}");
        let request_body = serde_json::to_string(&space)?;
        self.transport.put(&url, request_body)
    }

    /// Delete a specific space.
    pub fn delete_space(&self, space_id: &str) -> Result<types::EmptyResponse, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/space/{space_id}");
        self.transport.delete(&url)
    }

    /// Get a specific space.
    pub fn get_space(&self, space_id: &str) -> Result<types::Space, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/space/{space_id}");
        self.transport.get(&url)
    }
}
