use crate::{transport, types};

/// Goals Trait for the ClickUp API.
pub struct GoalsTraitTransporter {
    transport: crate::transport::Transport,
}

impl GoalsTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    // TODO: Add folders return paramater
    // TODO: Implement goals key results
    /// Get all goals.
    pub fn get_goals(&self, team_id: u64) -> Result<types::Goals, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/goal");
        self.transport.get(&url)
    }

    /// Create a goal.
    pub fn create_goal(
        &self,
        team_id: u64,
        goal: types::CreateGoal,
    ) -> Result<types::GoalContainer, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/team/{team_id}/goal");
        let request_body = serde_json::to_string(&goal)?;
        self.transport.post(&url, request_body)
    }

    /// Get a specific goal.
    pub fn get_goal(&self, goal_id: &str) -> Result<types::GoalContainer, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/goal/{goal_id}");
        self.transport.get(&url)
    }

    /// Delete a specific goal.
    pub fn delete_goal(&self, goal_id: &str) -> Result<types::EmptyResponse, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/goal/{goal_id}");
        self.transport.delete(&url)
    }

    /// Update a specific goal.
    pub fn update_goal(
        &self,
        goal_id: &str,
        goal: types::UpdateGoal,
    ) -> Result<types::GoalContainer, transport::Error> {
        let url = format!("https://api.clickup.com/api/v2/goal/{goal_id}");
        let request_body = serde_json::to_string(&goal)?;
        self.transport.put(&url, request_body)
    }
}
