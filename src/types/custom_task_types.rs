use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomItem {
    pub id: u64,
    pub name: String,
    pub name_plural: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomItems {
    pub items: Vec<CustomItem>,
}
