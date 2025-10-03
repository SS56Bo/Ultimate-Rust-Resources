#[macro_use] extern crate rocket;

#[get("/greet")]
fn greeting() -> &'static str {
    "hello world!"
}

#[launch]
fn rocket()->_ {
    rocket::build().mount("/", routes![greeting])
}
