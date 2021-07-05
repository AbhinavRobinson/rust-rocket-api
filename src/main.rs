// enable macros
#![feature(decl_macro)]

// IMPORTS
use rocket::*;

// Local Import
mod routes;

// INIT
// rocket server
fn main() {
    // connect to db and create table
    connect_db();

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

// connects to db and creates table if not exists.
fn connect_db() {
    // connect to sqlite
    let db_conn = rusqlite::Connection::open("data.sqlite").unwrap();

    // create table if no exists
    db_conn
        .execute(
            "create table if not exists todo_list (
                id integer primary key,
                item varchar(64) not null
            );",
            [],
        )
        .unwrap();
}
