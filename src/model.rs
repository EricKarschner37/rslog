use rocket::{Build, Rocket, fairing, serde::{Deserialize, Serialize}};
use rocket_db_pools::{Database, sqlx::{self, FromRow}};

#[derive(Database)]
#[database("rslog")]
pub struct DB(sqlx::PgPool);

#[derive(FromRow, FromForm, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub title: String,
    pub text: String,
}

#[derive(FromRow, FromForm, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostPointer {
    pub id: i32,
    pub title: String,
    pub url: String,
}

pub async fn run_db_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match DB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}
