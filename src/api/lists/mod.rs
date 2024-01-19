use crate::types;

pub struct ListsTraitTransporter {
    transport: crate::transport::Transport,
}

impl ListsTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_lists(&self, folder_id: &str) -> Result<types::Lists, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{}/list", folder_id);
        self.transport.get(&url)
    }

    pub fn create_list(&self, folder_id: &str, name: &str) -> Result<types::Lists, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn update_list(&self, list_id: &str, name: &str) -> Result<types::Lists, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn delete_list(&self, list_id: &str) -> Result<types::Lists, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn get_list(&self, list_id: &str) -> Result<types::Lists, Box<dyn std::error::Error>> {
        todo!()
    }
}
