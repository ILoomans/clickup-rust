use crate::{transport, types};

/// Members Trait for the ClickUp API.
pub struct MembersTraitTransporter {
    transport: crate::transport::Transport,
}

impl MembersTraitTransporter {
    /// Create a new instance of the Members Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get all task members.
    pub fn get_task_members(&self, task_id: &str) -> Result<types::Members, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}/member");
        self.transport.get(&url)
    }

    /// Get all list members.
    pub fn get_list_members(&self, list_id: u64) -> Result<types::Members, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/member");
        self.transport.get(&url)
    }
}
