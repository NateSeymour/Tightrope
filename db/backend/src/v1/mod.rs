pub mod metric;
pub mod user;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes: Vec<rocket::Route> = Vec::new();

    routes.extend(metric::routes());
    routes.extend(user::routes());

    routes
}