// enable macros
#![feature(decl_macro)]

// get macros from rocket
#[macro_use]
extern crate rocket;

// get macro from get
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

// init rocket server
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
