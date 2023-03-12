use rocket::http;
use auth::AuthProvider;
use fitness_runtime::FitnessRuntime;
use fitness_runtime::FitnessMode;
use serde::Serialize;
use sqlx::MySqlPool;

mod entity;
mod fitness_runtime;
mod auth;
mod cors;

mod v1;

#[derive(Serialize)]
pub struct ServerInfo {
    name: &'static str,
    version: &'static str,
}

#[rocket::options("/<_..>")]
fn server_info() -> http::Status {
    http::Status::Ok
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Setup environment
    dotenvy::dotenv().unwrap();

    let runtime = FitnessRuntime {
        database_url: std::env::var("DATABASE_URL").unwrap(),
        mode: match std::env::var("MODE").unwrap().as_str() {
            "production" => FitnessMode::Production,
            _ => FitnessMode::Development,
        }
    };

    // Create database connection pool
    let db = MySqlPool::connect(runtime.database_url.as_str()).await.unwrap();

    // Create the auth provider 
    let auth_provider = AuthProvider::new().await;

    // Start server
    let _rocket = rocket::build()
        .attach(cors::CORS)
        .manage(db)
        .manage(runtime)
        .manage(auth_provider)
        .mount("/v1", v1::routes())
        .mount("/", rocket::routes![server_info])
        .launch()
        .await?;

    Ok(())
}