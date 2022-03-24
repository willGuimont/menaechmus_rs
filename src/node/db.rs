use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_nodes")]
pub struct DbConn(diesel::SqliteConnection);
