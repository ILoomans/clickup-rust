use crate::types;


pub struct GoalsTraitTransporter {
    transport: crate::transport::Transport,
}

impl GoalsTraitTransporter {
    pub fn new (transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    // TODO: Add folders return paramater
    // TODO: Implement goals key results 
    pub fn get_goals(&self, team_id: u64) -> Result<types::Goals, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{}/goal", team_id);
        self.transport.get(&url)
    }

    pub fn create_goal (&self, team_id: u64, goal: types::CreateGoal) -> Result<types::GoalContainer, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/team/{}/goal", team_id);
        let request_body = serde_json::to_string(&goal)?;
        self.transport.post(&url, request_body)

    }

    pub fn get_goal(&self, goal_id: &str) -> Result<types::GoalContainer, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/goal/{}", goal_id);
        self.transport.get(&url)
    }

    pub fn delete_goal(&self , goal_id: &str) -> Result<types::EmptyResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/goal/{}", goal_id);
        self.transport.delete(&url)
    }

    pub fn update_goal (&self, goal_id: &str, goal: types::UpdateGoal) -> Result<types::GoalContainer, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/goal/{}", goal_id);
        let request_body = serde_json::to_string(&goal)?;
        self.transport.put(&url, request_body)
    }
}