use derive_more::{Display, From};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeError;

// https://clickup.com/api/developer-portal/errors/
#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "Team is not authorized")]
    GenericError,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::Error::*;
        match *self {
            GenericError => None,
        }
    }
}

impl From<SerdeError> for Error {
    fn from(_err: SerdeError) -> Self {
        Error::GenericError
    }
}

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
        let mut body = response.text().await.unwrap();
        if body.is_empty() {
            body = "{}".to_string();
        }

        let processed_output = serde_json::from_str::<T>(&body).unwrap_or_else(|err| {
            println!("Error: {err}");
            panic!("Failed to parse response from ClickUp API: {body}")
        });
        Ok(processed_output)
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
        let mut body = response.text().await.unwrap();
        if body.is_empty() {
            body = "{}".to_string();
        }
        let processed_output = serde_json::from_str::<T>(&body).unwrap_or_else(|err| {
            println!("Error: {err}");
            panic!("Failed to parse response from ClickUp API: {body}")
        });
        Ok(processed_output)
    }

    pub fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let method = reqwest::Method::GET;
        let resp: T = self.execute_request(method, url).unwrap(); // .unwrap(
        Ok(resp)
    }

    pub fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        request_body: String,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let method = reqwest::Method::POST;
        let resp: T = self
            .execute_request_post(method, url, request_body)
            .unwrap(); // .unwrap(
        Ok(resp)
    }

    pub fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let method = reqwest::Method::DELETE;
        let resp: T = self.execute_request(method, url).unwrap(); // .unwrap(
        Ok(resp)
    }

    pub fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        request_body: String,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let method = reqwest::Method::PUT;
        let resp: T = self
            .execute_request_post(method, url, request_body)
            .unwrap(); // .unwrap(
        Ok(resp)
    }
}
