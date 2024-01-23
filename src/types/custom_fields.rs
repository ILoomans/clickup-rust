use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    id: String,
    name: String,
    r#type: String,
    date_created: String,
    hide_from_guests: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    pub fields: Vec<Field>,
}


