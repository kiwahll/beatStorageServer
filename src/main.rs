use db::Beat;
use rocket::http::Status;
use rocket_cors::CorsOptions;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;
pub mod db;

#[get("/")]
fn index() -> Json<Vec<Beat>> {
    let beats: Vec<Beat> = db::get_beats().unwrap();
    Json(beats)
}

#[get("/add/<title>/<url>")]
fn add(title: &str, url: &str) -> Result<String, (Status, String)> {
    match db::add_beat(title, url) {
        Ok(id) => Ok(format!("ID: {}", id)),
        Err(_) => {
            Err((Status::BadRequest, format!("Existiert bereits!")))
        }
    }
}

#[launch]
fn rocket() -> _ {
    db::init();

    let cors = CorsOptions::default()
        .to_cors()
        .expect("CORS config failed");

    rocket::build()
        .mount("/", routes!(index, add))
        .attach(cors)
}
