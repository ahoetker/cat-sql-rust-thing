use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Human {
    pub human_id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct HumanInput {
    pub name: String,
}
