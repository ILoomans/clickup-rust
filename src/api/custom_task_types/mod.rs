use crate::types;

pub struct CustomTaskTypesTraitTransporter {
    transport: crate::transport::Transport,
}

impl CustomTaskTypesTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_custom_task_types(
        &self,
        team_id: u64,
    ) -> Result<types::CustomItems, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/custom_item");
        self.transport.get(&url)
    }

    // TODO: Implement set Custom Field Value to accommodate for the multiple possibilties

    // TODO: Implement set Custom Field Value
}
