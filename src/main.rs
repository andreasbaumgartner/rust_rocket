#[macro_use]
extern crate rocket;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!";
    "please go to /test"
}

#[get("/test")]
fn hello_test() -> &'static str {
    "Hello, test!"
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello_test])
        .mount("/hello", routes![hello])
}
