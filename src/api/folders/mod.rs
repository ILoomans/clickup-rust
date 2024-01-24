use crate::types::{self, CreateFolder};

/// Folders Trait for the ClickUp API.
pub struct FoldersTraitTransporter {
    transport: crate::transport::Transport,
}

impl FoldersTraitTransporter {
    /// Create a new instance of the Folders Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all folders.
    pub fn get_folders(
        &self,
        space_id: &str,
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/space/{space_id}/folder");
        self.transport.get(&url)
    }

    /// Create a specific folder.
    pub fn create_folder(
        &self,
        space_id: &str,
        name: &str,
    ) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let folder = CreateFolder {
            name: name.to_string(),
        };

        let url = format!("https://api.clickup.com/api/v2/space/{space_id}/folder");
        let request_body = serde_json::to_string(&folder)?;
        self.transport.post(&url, request_body)
    }

    /// Update a specific folder.
    pub fn update_folder(
        &self,
        folder_id: &str,
        name: &str,
    ) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let folder = CreateFolder {
            name: name.to_string(),
        };

        let url = format!("https://api.clickup.com/api/v2/folder/{folder_id}");
        let request_body = serde_json::to_string(&folder)?;
        self.transport.put(&url, request_body)
    }

    /// Delete a specific folder.
    pub fn delete_folder(
        &self,
        folder_id: &str,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{folder_id}");
        self.transport.delete(&url)
    }

    /// Get a specific folder.
    pub fn get_folder(&self, folder_id: &str) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{folder_id}");
        self.transport.get(&url)
    }
}
