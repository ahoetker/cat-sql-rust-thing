use crate::{
    schema::human::{Human, HumanInput},
    CatHouseError, Result,
};
use sqlx::PgPool;

pub async fn fetch_humans(pool: &PgPool) -> Vec<Human> {
    let cats = sqlx::query_as!(Human, "SELECT * FROM humans;")
        .fetch_all(pool)
        .await
        .unwrap();
    cats
}

pub async fn fetch_human(pool: &PgPool, id: i32) -> Result<Human> {
    sqlx::query_as!(Human, "SELECT * FROM humans WHERE human_id = $1;", id)
        .fetch_one(pool)
        .await
        .map_err(|e| CatHouseError::SqlError { source: e })
}

pub async fn delete_human(pool: &PgPool, id: i32) -> Result<()> {
    match sqlx::query!("DELETE FROM humans WHERE human_id = $1;", id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(CatHouseError::SqlError { source: e }),
    }
}

pub async fn insert_human(pool: &PgPool, human: HumanInput) -> Result<Human> {
    sqlx::query_as!(
        Human,
        "
INSERT INTO humans (name)
VALUES ($1)
RETURNING *;
    ",
        human.name
    )
    .fetch_one(pool)
    .await
    .map_err(|e| CatHouseError::SqlError { source: e })
}
