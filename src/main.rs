// use std::fmt::format;
use rocket::tokio::time::{sleep, Duration};

#[macro_use] extern crate rocket;
#[get("/<one>/<two>")]
fn param(one: &str, two: u8) -> String {
    format!("param test string : {}, integer : {}", one, two)
}
#[get("/")]
async fn index() -> String {
    format!("home")
}
#[get("/<seconds>")]
async fn delay(seconds:u64) -> String{
    sleep(Duration::from_secs(seconds)).await;
    format!("delay {} seconds!",seconds)
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index])
        .mount("/param", routes![param])
        .mount("/delay",routes![delay])
}