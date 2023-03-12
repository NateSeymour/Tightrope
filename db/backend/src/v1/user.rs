use rocket::serde::json::Json;
use crate::auth::User;

#[rocket::get("/user/whoami")]
pub fn whoami(user: User) -> Json<User> { Json(user) }

pub fn routes() -> Vec<rocket::Route> { rocket::routes![whoami] }