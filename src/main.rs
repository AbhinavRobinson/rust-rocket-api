#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}