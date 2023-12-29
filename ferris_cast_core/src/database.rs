use std::path::Path;
use crate::config;
use rusqlite::Connection;

pub fn init() -> Result<(), anyhow::Error>{
    let db_path = Path::new(config::DATABASE_PATH);
    let conn = initialize_database(db_path)?;
    Ok(())
}

fn initialize_database(db_path: &Path) -> rusqlite::Result<Connection, anyhow::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute("CREATE TABLE IF NOT EXISTS podcasts (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT NOT NULL,
        link        TEXT NOT NULL,
        image_url   TEXT,
        last_build_date DATE NOT NULL
    )", [])?;

    conn.execute("CREATE TABLE IF NOT EXISTS episodes (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        author          TEXT NOT NULL,
        title           TEXT NOT NULL,
        podcast_id,     INTEGER NOT NULL
        description     TEXT,
        resource_linK   TEXT NOT NULL,
        FOREIGN KEY(podcast_id) REFERENCES podcasts(id)
    )", [])?;

    return Ok(conn);
}