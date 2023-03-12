use rocket::serde::json::Json;
use rocket::http;
use rocket::State;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use crate::auth::User;
use crate::entity::measurement::Measurement;
use crate::entity::metric::{Metric, MetricType};

#[derive(Serialize, Deserialize)]
pub struct MeasurementPostRequest {
    pub value: f64,
    pub millis: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementPostResponse {
    pub id: u64,
}

#[rocket::post("/metric/<metric_name>/measurement", data="<body>")]
pub async fn post_measurement(user: User, db: &State<MySqlPool>, body: Json<MeasurementPostRequest>, metric_name: String) -> Result<Json<MeasurementPostResponse>, http::Status> {
    let res = sqlx::query!(
            r#"INSERT INTO measurement(metric_id, username, millis, value) SELECT metric.metric_id,?,?,? FROM metric WHERE metric.name = ?"#,
            user.preferred_username,
            body.millis,
            body.value,
            metric_name
        )
        .execute(db.inner()).await;

    match res {
        Ok(val) => Ok(Json(MeasurementPostResponse { id: val.last_insert_id() })),
        Err(e) => {
            println!("{}", e);

            Err(http::Status::InternalServerError)
        },
    }
}

#[rocket::delete("/metric/<metric_name>/measurement/<id>")]
pub async fn delete_measurement(user: User, db: &State<MySqlPool>, metric_name: String, id: u64) -> http::Status {
    let res = sqlx::query!(
            "DELETE FROM measurement WHERE metric_id IN (SELECT metric_id FROM metric WHERE name = ?) AND measurement_id = ? AND username = ?",
            metric_name, id, user.preferred_username
        )
        .execute(db.inner()).await;

    match res {
        Ok(val) => {
            if val.rows_affected() > 0 {
                return http::Status::Ok;
            }

            http::Status::NotFound
        },
        Err(_) => http::Status::InternalServerError,
    }
}

#[rocket::get("/metric/<metric_name>/measurement/<id>")]
pub async fn get_measurement(user: User, db: &State<MySqlPool>, metric_name: String, id: u64) -> Result<Json<Measurement>, http::Status> {
    let res = sqlx::query_as!(Measurement,
            "SELECT * FROM measurement WHERE metric_id IN (SELECT metric_id FROM metric WHERE name = ?) AND username = ? AND measurement_id = ?",
            metric_name, user.preferred_username, id
        )
        .fetch_one(db.inner()).await.unwrap();

    Ok(Json(res))
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementPatchRequest {
    pub value: f64,
    pub millis: u64,
}

#[rocket::patch("/metric/<metric_name>/measurement/<id>", data="<body>")]
pub async fn patch_measurement(user: User, db: &State<MySqlPool>, metric_name: String, id: u64, body: Json<MeasurementPatchRequest>) -> http::Status {
    let _res = sqlx::query!(
            "UPDATE measurement SET value = ?, millis = ? WHERE metric_id IN (SELECT metric_id FROM metric WHERE name = ?) AND username = ? AND measurement_id = ?",
            body.value, body.millis, metric_name, user.preferred_username, id
        )
        .execute(db.inner()).await;

    http::Status::Accepted
}

#[rocket::get("/metric/<metric_name>/measurements")]
pub async fn get_metric_measurements(user: User, db: &State<MySqlPool>, metric_name: String) -> Result<Json<Vec<Measurement>>, http::Status> {
    let res = sqlx::query_as!(Measurement,
            "SELECT * FROM measurement WHERE metric_id IN (SELECT metric_id FROM metric WHERE name = ?) AND username = ? ORDER BY millis",
            metric_name, user.preferred_username
        )
        .fetch_all(db.inner()).await.unwrap();

    Ok(Json(res))
}

#[rocket::get("/metric/list")]
pub async fn get_metric_list(user: User, db: &State<MySqlPool>) -> Result<Json<Vec<Metric>>, http::Status> {
    /*
     * Fucking nightmare workaround because `sqlx` enum querying is broken as hell.
     * ref: https://github.com/launchbadge/sqlx/issues/1241
     */
    let res = sqlx::query_as!(Metric,
            r#"SELECT metric_id,
                      metric_type `metric_type: MetricType`,
                      username,
                      name,
                      description
               FROM metric WHERE username = ?"#,
            user.preferred_username
        )
        .fetch_all(db.inner()).await.unwrap();

    Ok(Json(res))
}

#[rocket::get("/metric/<metric_name>")]
pub async fn get_metric_by_name(user: User, db: &State<MySqlPool>, metric_name: String) -> Result<Json<Metric>, http::Status> {
    let res = sqlx::query_as!(Metric,
            r#"SELECT metric_id,
                      metric_type `metric_type: MetricType`,
                      username,
                      name,
                      description
               FROM metric WHERE (username = ?) AND name = ?"#,
            user.preferred_username,
            metric_name
        )
        .fetch_one(db.inner()).await.unwrap();

    Ok(Json(res))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        post_measurement, get_measurement, delete_measurement, patch_measurement,
        get_metric_measurements,
        get_metric_list,
        get_metric_by_name]
}