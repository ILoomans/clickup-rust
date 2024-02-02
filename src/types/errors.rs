use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickUpError {
    pub err: String,
    #[serde(rename = "ECODE")]
    pub e_code: String,
}
