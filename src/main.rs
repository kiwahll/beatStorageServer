use db::Beat;
use rocket::{http::Status, Response};
use rocket_cors::{CorsOptions};

#[macro_use] extern crate rocket;
pub mod db;

#[get("/")]
fn index() -> String {
    let beats: Vec<Beat> = db::get_beats().unwrap();
    format!("{:?}", beats)
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
