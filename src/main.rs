// enable macros
#![feature(decl_macro)]

// IMPORTS
use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;

// STRUCTURES
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

// ROUTES
// / route
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

// /todo route
#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<TodoList>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to DB".into()),
    };

    let mut statement = match db_conn.prepare("select id, item from todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<TodoItem>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(TodoList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        _ => Err("DB Error: Failed to fetch items".into()),
    }
}

// /todo route
#[post("/todo", format = "json", data = "<item>")]
fn post_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to DB".into()),
    };

    let mut statement = match db_conn.prepare("insert into todo_list (id, item) values (null, $1);")
    {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        _ => Err("DB Error: Failed to insert items".into()),
    }
}

// /todo/id route
#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to DB".into()),
    };

    let mut statement = match db_conn.prepare("delete from todo_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        _ => Err("DB Error: Failed to insert items".into()),
    }
}

// INIT
// rocket server
fn main() {
    // db scope
    {
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
    } // end of db scope

    // mount to default route
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                fetch_all_todo_items,
                post_todo_item,
                delete_todo_item
            ],
        )
        .launch();
}
