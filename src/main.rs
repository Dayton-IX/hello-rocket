#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
