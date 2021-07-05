use serde::Serialize;

// STRUCTURES
#[derive(Serialize)]
pub struct TodoList {
  pub items: Vec<TodoItem>,
}

#[derive(Serialize)]
pub struct TodoItem {
  pub id: i64,
  pub item: String,
}

#[derive(Serialize)]
pub struct StatusMessage {
  pub message: String,
}
