use crate::types::{self, CreateFolder};

pub struct FoldersTraitTransporter {
    transport: crate::transport::Transport,
}

impl FoldersTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_folders(
        &self,
        space_id: &str,
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/space/{}/folder", space_id);
        self.transport.get(&url)
    }

    pub fn create_folder(
        &self,
        space_id: &str,
        name: &str,
    ) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let folder = CreateFolder {
            name: name.to_string(),
        };

        let url = format!("https://api.clickup.com/api/v2/space/{}/folder", space_id);
        let request_body = serde_json::to_string(&folder)?;
        self.transport.post(&url, request_body)
    }

    pub fn update_folder(
        &self,
        folder_id: &str,
        name: &str,
    ) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let folder = CreateFolder {
            name: name.to_string(),
        };

        let url = format!("https://api.clickup.com/api/v2/folder/{}", folder_id);
        let request_body = serde_json::to_string(&folder)?;
        self.transport.put(&url, request_body)
    }

    pub fn delete_folder(
        &self,
        folder_id: &str,
    ) -> Result<types::EmpptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{}", folder_id);
        self.transport.delete(&url)
    }

    pub fn get_folder(&self, folder_id: &str) -> Result<types::Folder, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{}", folder_id);
        self.transport.get(&url)
    }
}
