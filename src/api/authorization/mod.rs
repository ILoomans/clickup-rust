use crate::{transport, types};

pub struct AuthorizationTraitTransporter {
    transport: crate::transport::Transport,
}

impl AuthorizationTraitTransporter {
    /// Create a new instance of the Authorization Trait Transporter.
    pub fn new(transport: crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Get an access token.
    pub fn get_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
    ) -> Result<types::AccessToken, transport::Error> {
        let url = format!(
            "https://api.clickup.com/api/v2/oauth/token?client_id={client_id}&client_secret={client_secret}&code={code}"
        );
        self.transport.post(&url, "{}".to_string())
    }

    /// Get an authorized user.
    pub fn get_authorized_user(&self) -> Result<types::AuthorizedUser, transport::Error> {
        let url = "https://api.clickup.com/api/v2/user".to_string();
        self.transport.get(&url)
    }
}
