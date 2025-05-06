use db::Beat;

#[macro_use] extern crate rocket;
pub mod db;

#[get("/")]
fn index() -> String {
    let beats: Vec<Beat> = db::get_beats().unwrap();
    format!("{:?}", beats)
}

#[get("/add/<title>/<url>")]
fn echo(title: &str, url: &str) -> String {
    let id: i64 = db::add_beat(title, url).unwrap();
    format!("ID: {}", id)
}

#[launch]
fn rocket() -> _ {
    db::init();
    rocket::build().mount("/", routes!(index, echo))
}
