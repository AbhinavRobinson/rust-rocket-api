// enable macros
#![feature(decl_macro)]

// get macros from rocket
#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Serialize)]
struct TodoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

// get macro from get
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

// init rocket server
fn main() {
    // db scope
    {
        // connect to sqlite
        let db_conn = rusqlite::Connection::open("data.sqlite").unwrap();

        // create table if no exists
        db_conn
            .execute(
                "create table if not exists todo (
                    id integer primary key,
                    item varchar(64) not null
                );",
                [],
            )
            .unwrap();
    } // end of db scope

    // mount to default route
    rocket::ignite().mount("/", routes![index]).launch();
}
