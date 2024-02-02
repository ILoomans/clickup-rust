use crate::transport;
use crate::types::{self, CreateList};
/// Lists Trait for the ClickUp API.
pub struct ListsTraitTransporter {
    transport: crate::transport::Transport,
}

impl ListsTraitTransporter {
    /// Create a new instance of the Lists Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all lists.
    pub fn get_lists(&self, folder_id: &str) -> Result<types::Lists, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/folder/{folder_id}/list");
        self.transport.get(&url)
    }

    /// Create a specific list.
    pub fn create_list(
        &self,
        folder_id: &str,
        list: CreateList,
    ) -> Result<types::List, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/folder/{folder_id}/list");
        let request_body = serde_json::to_string(&list)?;
        self.transport.post(&url, request_body)
    }

    /// Update a specific list.
    pub fn update_list(
        &self,
        list_id: &str,
        list: CreateList,
    ) -> Result<types::List, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}");
        let request_body = serde_json::to_string(&list)?;
        self.transport.put(&url, request_body)
    }

    /// Delete a specific list.
    pub fn delete_list(&self, list_id: &str) -> Result<types::EmptyResponse, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}");
        self.transport.delete(&url)
    }

    /// Get a specific list.
    pub fn get_list(&self, list_id: &str) -> Result<types::List, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}");
        self.transport.get(&url)
    }
}
