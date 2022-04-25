use diesel::prelude::*;
use rocket_sync_db_pools::database;

use crate::node::{ContentTypeImpl, Node};
use crate::Peer;

use super::schema::blockchains;
use super::schema::blocks;
use super::schema::nodes;
use super::schema::peers;

#[database("sqlite_nodes")]
pub struct DbConn(SqliteConnection);

impl DbConn {
    pub fn inner(&self) -> SqliteConnection {
        self.inner()
    }
}

#[derive(Queryable)]
struct DbNode {
    pub id: i32,
    pub url: String,
    pub start_block_id: i32,
}

#[derive(Insertable)]
#[table_name = "nodes"]
struct NewDbNode<'a> {
    pub url: &'a str,
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

impl DbPeer {
    pub fn to_peer(self) -> Peer {
        Peer::new(self.url)
    }
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewDbPeer<'a> {
    pub url: &'a str,
}

pub fn insert_peer(conn: &SqliteConnection, url: &str) {
    let new_peer = NewDbPeer { url };
    diesel::insert_into(peers::table)
        .values(&new_peer)
        .execute(conn)
        .expect("Error saving peer");
}

pub fn get_peers(conn: &SqliteConnection) -> Vec<Peer> {
    peers::table
        .load::<DbPeer>(conn)
        .expect("Error loading peers")
        .into_iter()
        .map(DbPeer::to_peer)
        .collect()
}

pub fn load_node(conn: &diesel::SqliteConnection) -> Option<Node<ContentTypeImpl>> {
    let n = nodes::table
        .load::<DbNode>(conn)
        .expect("Error loading nodes")
        .into_iter()
        .nth(0);

    let ps = get_peers(conn);


    // Node::load(url)
    unimplemented!()
}
