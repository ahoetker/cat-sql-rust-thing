#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;

mod schema;
mod service;

use schema::{
    cat::{Cat, CatInput},
    human::{Human, HumanInput},
};
use service::{cat, human};

use rocket::serde::json::Json;
use rocket::State;
use rocket::{http::Status, response, Request};
use sqlx::PgPool;
use thiserror::Error;

// Error ----------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum CatHouseError {
    #[error("Database Error {source:?}")]
    SqlError {
        #[from]
        source: sqlx::Error,
    },
}

// Stolen from https://stuarth.github.io/rocket-error-handling/
impl<'r, 'o: 'r> response::Responder<'r, 'o> for CatHouseError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        println!("{:?}", self);

        match self {
            CatHouseError::SqlError { source } => {
                match source {
                    // Return 404
                    sqlx::Error::RowNotFound => Status::NotFound.respond_to(req),
                    _ => Status::InternalServerError.respond_to(req),
                }
            }
        }
    }
}

type Result<T> = std::result::Result<T, CatHouseError>;

// Routes ----------------------------

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/cat")]
async fn get_cats(pool: &State<PgPool>) -> Json<Vec<Cat>> {
    Json(cat::fetch_cats(&pool).await)
}

#[get("/cat/<id>")]
async fn get_cat(pool: &State<PgPool>, id: i32) -> Result<Json<Cat>> {
    let cat = cat::fetch_cat(&pool, id).await?;
    Ok(Json(cat))
}

#[post("/cat", format = "json", data = "<cat>")]
async fn create_cat(pool: &State<PgPool>, cat: Json<CatInput>) -> Result<Json<Cat>> {
    let created = cat::insert_cat(pool, cat.into_inner()).await?;
    Ok(Json(created))
}

#[delete("/cat/<id>")]
async fn delete_cat(pool: &State<PgPool>, id: i32) -> Result<()> {
    cat::delete_cat_db(pool, id).await
}


#[get("/human")]
async fn get_humans(pool: &State<PgPool>) -> Json<Vec<Human>> {
    Json(human::fetch_humans(pool).await)
}

#[get("/human/<id>")]
async fn get_human(pool: &State<PgPool>, id: i32) -> Result<Json<Human>> {
    let human = human::fetch_human(&pool, id).await?;
    Ok(Json(human))
}

#[post("/human", format = "json", data = "<human>")]
async fn create_human(pool: &State<PgPool>, human: Json<HumanInput>) -> Result<Json<Human>> {
    let created = human::insert_human(pool, human.into_inner()).await?;
    Ok(Json(created))
}

#[launch]
async fn rocket() -> _ {
    let pool = PgPool::connect(dotenv!("DATABASE_URL")).await.unwrap();

    println!("Running SQL migrations...");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    rocket::build().manage(pool).mount(
        "/",
        routes![
            index,
            get_cats,
            get_cat,
            create_cat,
            delete_cat,
            get_humans,
            get_human,
            create_human
        ],
    )
}
