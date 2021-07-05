#[path = "utils/mod.rs"]
mod utils;

// tests connection to db server
#[test]
#[should_panic]
pub fn connect_test() {
  utils::connect_db();
}
