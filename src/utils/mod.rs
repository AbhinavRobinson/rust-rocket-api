// connects to db and creates table if not exists.
pub fn connect_db() {
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
