use diesel::prelude::*;
use rocket_sync_db_pools::database;

use crate::node::{ContentTypeImpl, Node};

use super::schema::blockchains;
use super::schema::blocks;
use super::schema::nodes;
use super::schema::peers;

#[database("sqlite_nodes")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Queryable)]
struct DbNode {
    pub id: i32,
    pub url: String,
    pub timeout_ms: i32,
    pub start_block_id: i32,
}

#[derive(Insertable)]
#[table_name = "nodes"]
struct NewDbNode<'a> {
    pub url: &'a str,
    pub timeout_ms: i32,
    pub start_block_id: i32,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name = "blockchains"]
struct DbBlockchain {
    pub head_block: i32,
    pub tail_block: i32,
}

#[derive(Queryable)]
struct DbBlock {
    pub id: i32,
    pub hash: String,
    pub prev_hash: String,
    pub content: String,
    pub nonce: String,
}

#[derive(Insertable)]
#[table_name = "blocks"]
struct NewDbBlock<'a> {
    pub hash: &'a str,
    pub prev_hash: &'a str,
    pub content: &'a str,
    pub nonce: &'a str,
}

#[derive(Queryable)]
struct DbPeer {
    pub id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewDbPeer<'a> {
    pub url: &'a str,
}

pub fn insert_peer(conn: &diesel::SqliteConnection, url: &str) {
    let new_peer = NewDbPeer { url };
    diesel::insert_into(peers::table)
        .values(&new_peer)
        .execute(conn)
        .expect("Error saving peer");
}

pub fn load_node(conn: &diesel::SqliteConnection) -> Node<ContentTypeImpl> {
    unimplemented!()
}
