// enable macros
#![feature(decl_macro)]

// IMPORTS
use rocket::*;

// Local Import
mod routes;
mod utils;

// INIT
// rocket server
fn main() {
    // connect to db and create table
    utils::connect_db();

    // mount to default route
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::fetch_all_todo_items,
                routes::post_todo_item,
                routes::delete_todo_item
            ],
        )
        .launch();
}
