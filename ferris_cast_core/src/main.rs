use rusqlite::{Connection, Result};
fn main() {
    let conn = Connection::open("my_database.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS podcast (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  episode_count   INTEGER
                  )",
        [],
    ).unwrap();

}