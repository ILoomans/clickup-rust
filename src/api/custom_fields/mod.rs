use crate::types;

/// Custom Fields Trait for the ClickUp API.
pub struct CustomFieldsTraitTransporter {
    transport: crate::transport::Transport,
}

impl CustomFieldsTraitTransporter {
    /// Create a new instance of the Custom Fields Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all accessible custom fields.
    pub fn get_accessible_custom_fields(
        &self,
        list_id: u64,
    ) -> Result<types::Fields, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/field");
        self.transport.get(&url)
    }
}
