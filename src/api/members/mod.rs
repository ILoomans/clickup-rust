use crate::types;

pub struct MembersTraitTransporter {
    transport: crate::transport::Transport,
}

impl MembersTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    pub fn get_task_members(
        &self,
        task_id: &str,
    ) -> Result<types::Members, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/task/{task_id}/member");
        self.transport.get(&url)
    }

    pub fn get_list_members(
        &self,
        list_id: u64,
    ) -> Result<types::Members, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/list/{list_id}/member");
        self.transport.get(&url)
    }
}
