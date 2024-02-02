use crate::types::ClickUpError;
use derive_more::{Display, From};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeError;
#[derive(Debug, Display, From, PartialEq)]

pub enum Error {
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Team not authorized")]
    TeamNotAuthorized,
    #[display(fmt = "Token not found")]
    TokenNotFound,
    #[display(fmt = "Authorization Header Required")]
    AuthorizationHeaderRequired,
    #[display(fmt = "Client Not Found")]
    ClientNotFound,
    #[display(fmt = "Rediect URIdoes not match the redirect uris of this application")]
    RedirectUriNotMatching,
    #[display(fmt = "Webhook Configuration Exists")]
    WebhookConfigurationExists,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::Error::*;
        match *self {
            GenericError => self.source(),
            TeamNotAuthorized => self.source(),
            TokenNotFound => self.source(),
            AuthorizationHeaderRequired => self.source(),
            ClientNotFound => self.source(),
            RedirectUriNotMatching => self.source(),
            WebhookConfigurationExists => self.source(),
        }
    }
}

impl From<SerdeError> for Error {
    fn from(_err: SerdeError) -> Self {
        Error::GenericError
    }
}

// create map

#[derive(Clone, Debug)]
pub struct Transport {
    pub client: reqwest::Client,
    pub api_key: String,
}

/// Transport is a wrapper around reqwest::Client
impl Transport {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Transport { client, api_key }
    }

    fn match_error(&self, error: &str) -> Error {
        match error {
            "OAUTH_023" | "OAUTH_026" | "OAUTH_027" | "OAUTH_029" | "OAUTH_045" => {
                Error::TeamNotAuthorized
            }
            "OAUTH_019" | "OAUTH_021" | "OAUTH_025" | "OAUTH_077" => Error::TokenNotFound,
            "OAUTH_017" => Error::AuthorizationHeaderRequired,
            "OAUTH_010" => Error::ClientNotFound,
            "OAUTH_007" => Error::RedirectUriNotMatching,
            "OAUTH_009" => Error::WebhookConfigurationExists,
            _ => Error::GenericError,
        }
    }
    fn process_output<T: DeserializeOwned>(&self, mut body: String) -> Result<T, Error> {
        if body.is_empty() {
            body = "{}".to_string();
        }
        serde_json::from_str::<T>(&body).map_err(|_err| {
            let click_up_error_match = serde_json::from_str::<ClickUpError>(&body);
            match click_up_error_match {
                Ok(click_up_error) => self.match_error(click_up_error.e_code.as_str()),
                Err(_) => Error::GenericError,
            }
        })
    }

    #[tokio::main]
    async fn execute_request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> Result<T, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", self.api_key.parse().unwrap());

        let request = self.client.request(method, url).headers(headers);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();
        self.process_output(body)
    }

    #[tokio::main]
    async fn execute_request_post<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
        request_body: String,
    ) -> Result<T, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", self.api_key.parse().unwrap());

        let request = self
            .client
            .request(method, url)
            .headers(headers)
            .body(request_body);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();
        self.process_output(body)
    }

    pub fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let method = reqwest::Method::GET;
        self.execute_request(method, url) // .unwrap(
    }

    pub fn post<T: DeserializeOwned>(&self, url: &str, request_body: String) -> Result<T, Error> {
        let method = reqwest::Method::POST;
        self.execute_request_post(method, url, request_body)
    }

    pub fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let method = reqwest::Method::DELETE;
        self.execute_request(method, url) // .unwrap(
    }

    pub fn put<T: DeserializeOwned>(&self, url: &str, request_body: String) -> Result<T, Error> {
        let method = reqwest::Method::PUT;
        self.execute_request_post(method, url, request_body) // .unwrap(
    }
}
