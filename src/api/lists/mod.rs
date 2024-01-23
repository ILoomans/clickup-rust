use crate::types::{self, CreateList};

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

    pub fn create_list(
        &self,
        folder_id: &str,
        list: CreateList,
    ) -> Result<types::List, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/folder/{}/list", folder_id);
        let request_body = serde_json::to_string(&list)?;
        self.transport.post(&url, request_body)
    }

    pub fn update_list(
        &self,
        list_id: &str,
        list: CreateList,
    ) -> Result<types::List, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}", list_id);
        let request_body = serde_json::to_string(&list)?;
        self.transport.put(&url, request_body)
    }

    pub fn delete_list(
        &self,
        list_id: &str,
    ) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}", list_id);
        self.transport.delete(&url)
    }

    pub fn get_list(&self, list_id: &str) -> Result<types::List, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}", list_id);
        self.transport.get(&url)
    }
}
