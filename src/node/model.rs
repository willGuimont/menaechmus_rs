use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

use crate::node::{ContentTypeImpl, Node};
use crate::schema;

#[database("sqlite_nodes")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Queryable)]
struct DbNode {}

pub fn load_node(conn: &diesel::SqliteConnection) -> Node<ContentTypeImpl> {
    unimplemented!()
}
