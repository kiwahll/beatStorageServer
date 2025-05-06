use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Error, Result};
use std::sync::Mutex;

#[derive(Debug)]
pub struct Beat {
    id: i64,
    title: String,
    url: String,
    datetime: DateTime<Utc>,
}

lazy_static! {
    static ref CLIENT: Mutex<Connection> =
        Mutex::new(Connection::open("beats.db").expect("Failed to open database"));
}

pub fn init() {
    let conn = CLIENT.lock().unwrap();
    let _ = conn.execute(
        "CREATE TABLE beats (
            id    INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            url  TEXT NOT NULL UNIQUE,
            datetime TEXT NOT NULL
        )",
        (),
    );
}

pub fn add_beat(title: &str, url: &str) -> Result<i64, rusqlite::Error> {
    let conn = CLIENT.lock().unwrap();
    match conn.execute(
        "INSERT INTO beats (title, url, datetime) VALUES (?1, ?2, ?3)",
        params![title, url, chrono::Utc::now().to_rfc3339()],
    ) {
        Ok(_) => {
            Ok(conn.last_insert_rowid())
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub fn get_beats() -> Result<Vec<Beat>, rusqlite::Error> {
    let conn = CLIENT.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, url, datetime FROM beats")?;
    let beats_iter = stmt
        .query_map([], |row| {
            let datetime_str: String = row.get(3)?;
            let datetime: DateTime<Utc> = DateTime::parse_from_rfc3339(&datetime_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap();

            Ok(Beat {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                datetime,
            })
        })
        .expect("Konvertierungs Fehler!")
        .collect::<Vec<_>>();

    let mut beats = Vec::new();
    for beat in beats_iter {
        beats.push(beat?); // Fehler werden hier aufgelöst
    }

    Ok(beats)
}
