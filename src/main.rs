#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;

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

    rocket::build().mount("/", routes![login])
}
