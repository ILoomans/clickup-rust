
use crate::types;

pub struct CustomFieldsTraitTransporter {
    transport: crate::transport::Transport,
}


impl CustomFieldsTraitTransporter {
    
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_accessible_custom_fields(&self, list_id: u64) -> Result<types::Fields, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{}/field", list_id);
        self.transport.get(&url)
    }
}