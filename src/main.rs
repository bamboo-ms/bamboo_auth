#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

extern crate diesel;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
}

#[post("/login", format = "application/json", data = "<user>")]
async fn login(user: Json<User>) {
    println!("{}", user.username);
}

#[launch]
fn launch() -> _ {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    println!(
        "{:?}",
        settings.try_into::<HashMap<String, String>>().unwrap()
    );

    rocket::build().mount("/", routes![login])
}
