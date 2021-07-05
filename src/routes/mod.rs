// IMPORTS
use rocket::*;
use rocket_contrib::json::Json;

// Import structs
mod structs;

// ROUTES
// / route
#[get("/")]
pub fn index() -> &'static str {
  "Hello, Rocket!"
}

// /todo route
#[get("/todo")]
pub fn fetch_all_todo_items() -> Result<Json<structs::TodoList>, String> {
  let db_conn = match rusqlite::Connection::open("data.sqlite") {
    Ok(connection) => connection,
    Err(_) => return Err("Failed to connect to DB".into()),
  };

  let mut statement = match db_conn.prepare("select id, item from todo_list;") {
    Ok(statement) => statement,
    Err(_) => return Err("Failed to prepare query".into()),
  };

  let results = statement.query_map([], |row| {
    Ok(structs::TodoItem {
      id: row.get(0)?,
      item: row.get(1)?,
    })
  });

  match results {
    Ok(rows) => {
      let collection: rusqlite::Result<Vec<structs::TodoItem>> = rows.collect();

      match collection {
        Ok(items) => Ok(Json(structs::TodoList { items })),
        Err(_) => Err("Could not collect items".into()),
      }
    }
    _ => Err("DB Error: Failed to fetch items".into()),
  }
}

// /todo route
#[post("/todo", format = "json", data = "<item>")]
pub fn post_todo_item(item: Json<String>) -> Result<Json<structs::StatusMessage>, String> {
  let db_conn = match rusqlite::Connection::open("data.sqlite") {
    Ok(connection) => connection,
    Err(_) => return Err("Failed to connect to DB".into()),
  };

  let mut statement = match db_conn.prepare("insert into todo_list (id, item) values (null, $1);") {
    Ok(statement) => statement,
    Err(_) => return Err("Failed to prepare query".into()),
  };

  let results = statement.execute(&[&item.0]);

  match results {
    Ok(rows_affected) => Ok(Json(structs::StatusMessage {
      message: format!("{} rows inserted!", rows_affected),
    })),
    _ => Err("DB Error: Failed to insert items".into()),
  }
}

// /todo/id route
#[delete("/todo/<id>")]
pub fn delete_todo_item(id: i64) -> Result<Json<structs::StatusMessage>, String> {
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
    Ok(rows_affected) => Ok(Json(structs::StatusMessage {
      message: format!("{} rows deleted!", rows_affected),
    })),
    _ => Err("DB Error: Failed to insert items".into()),
  }
}
