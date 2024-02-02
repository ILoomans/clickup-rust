use crate::{transport, types};

/// Custom Task Types Trait for the ClickUp API.
pub struct CustomTaskTypesTraitTransporter {
    transport: crate::transport::Transport,
}

impl CustomTaskTypesTraitTransporter {
    /// Create a new instance of the Custom Task Types Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all custom task types.
    pub fn get_custom_task_types(
        &self,
        team_id: u64,
    ) -> Result<types::CustomItems, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/custom_item");
        self.transport.get(&url)
    }

    // TODO: Implement set Custom Field Value to accommodate for the multiple possibilties

    // TODO: Implement set Custom Field Value
}
