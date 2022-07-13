// use std::fmt::format;
use rocket::tokio::time::{sleep, Duration};
use rocket::http::CookieJar;
#[macro_use] extern crate rocket;

#[get("/<one>/<two>")]
fn param(one: &str, two: u8) -> String {
    format!("param test string : {}, integer : {}", one, two)
}
#[get("/")]
fn index() -> String {
    format!("home")
}
#[get("/<seconds>")]
async fn delay(seconds:u64) -> String{
    sleep(Duration::from_secs(seconds)).await;
    format!("delay {} seconds!",seconds)
}
#[get("/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index])
        .mount("/every",routes![everything])
        .mount("/param", routes![param])
        .mount("/delay",routes![delay])
}