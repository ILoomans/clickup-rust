use crate::types;

pub struct AuthorizationTraitTransporter {
    transport: crate::transport::Transport,
}

impl AuthorizationTraitTransporter {
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    // TODO: Test this function
    pub fn get_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
    ) -> Result<types::AccessToken, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.clickup.com/api/v2/oauth/token?client_id={}&client_secret={}&code={}",
            client_id, client_secret, code
        );
        self.transport.post(&url, "{}".to_string())
    }

    pub fn get_authorized_user(&self) -> Result<types::AuthorizedUser, Box<dyn std::error::Error>> {
        let url = format!("https://api.clickup.com/api/v2/user");
        self.transport.get(&url)
    }
}
