use crate::{
    schema::cat::{Cat, CatInput},
    CatHouseError, Result,
};
use sqlx::PgPool;

pub async fn fetch_cats(pool: &PgPool) -> Vec<Cat> {
    let cats = sqlx::query_as!(Cat, "SELECT * FROM cats;")
        .fetch_all(pool)
        .await
        .unwrap();
    cats
}

pub async fn fetch_cat(pool: &PgPool, id: i32) -> Result<Cat> {
    sqlx::query_as!(Cat, "SELECT * FROM cats WHERE cat_id = $1;", id)
        .fetch_one(pool)
        .await
        .map_err(|e| CatHouseError::SqlError { source: e })
}

pub async fn update_cat_db(pool: &PgPool, cat: &CatInput) -> Result<Cat> {
    sqlx::query_as!(
        Cat,
        "
UPDATE cats
SET
    name = $1,
    kind = $2,
    age = $3,
    sex = $4,
    favorite_foods = $6,
    human_id = $7 WHERE cat_id = $8
RETURNING *;",
        cat.name,
        cat.kind,
        cat.age,
        cat.sex,
        &cat.favorite_foods,
        cat.human_id
        cat.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| CatHouseError::SqlError { source: e })
}

pub async fn delete_cat_db(pool: &PgPool, id: i32) -> Result<()> {
    match sqlx::query!("DELETE FROM cats WHERE cat_id = $1;", id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(CatHouseError::SqlError { source: e }),
    }
}

pub async fn insert_cat(pool: &PgPool, cat: CatInput) -> Result<Cat> {
    sqlx::query_as!(
        Cat,
        "
INSERT INTO cats (name, kind, age, sex, favorite_foods, human_id)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;
    ",
        cat.name,
        cat.kind,
        cat.age,
        cat.sex,
        &cat.favorite_foods,
        cat.human_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| CatHouseError::SqlError { source: e })
}
