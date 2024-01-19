use crate::types;

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
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn update_folder(
        &self,
        folder_id: &str,
        name: &str,
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_folder(
        &self,
        folder_id: &str,
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn get_folder(
        &self,
        folder_id: &str,
    ) -> Result<types::Folders, Box<dyn std::error::Error>> {
        todo!()
    }
}
