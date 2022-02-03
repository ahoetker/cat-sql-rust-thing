use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Cat {
    pub cat_id: i32,
    pub name: String,
    pub kind: String,
    pub age: i32,
    pub sex: String,
    pub favorite_foods: Vec<String>,
    pub human_id: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct CatInput {
    pub name: String,
    pub kind: String,
    pub age: i32,
    pub sex: String,
    pub favorite_foods: Vec<String>,
    pub human_id: Option<i32>,
}
