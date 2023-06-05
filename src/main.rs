#[macro_use]
extern crate rocket;

mod api;
mod tera;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!";
    "please go to /test"
}

#[get("/test")]
fn hello_test() -> &'static str {
    "Hello, test!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello_test])
        .mount("/", routes![tera::about])
        .mount("/index", routes![tera::index])
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera);
        }))
}
