use diesel::prelude::*;
use rocket_sync_db_pools::database;
use menaechmus::{Block, Blockchain};

use crate::node::{ContentTypeImpl, Node};
use crate::Peer;

use super::schema::blockchains;
use super::schema::blocks;
use super::schema::nodes;
use super::schema::peers;

#[database("sqlite_nodes")]
pub struct DbConn(pub SqliteConnection);

#[derive(Queryable, Clone)]
struct DbNode {
    pub id: i32,
    pub url: String,
    pub start_block_id: i32,
    pub hash_starting_pattern: String,
}

#[derive(Insertable)]
#[table_name = "nodes"]
struct NewDbNode<'a> {
    pub url: &'a str,
    pub start_block_id: i32,
    pub hash_starting_pattern: &'a str,
}

#[derive(Queryable, Insertable, Clone)]
#[table_name = "blockchains"]
struct DbBlockchain {
    pub head_block: i32,
    pub tail_block: i32,
}

#[derive(Queryable, Clone)]
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
struct NewDbPeer<'a> {
    pub url: &'a str,
}

pub fn insert_peer(conn: &SqliteConnection, url: &str) {
    let new_peer = NewDbPeer { url };
    diesel::insert_into(peers::table)
        .values(&new_peer)
        .execute(conn)
        .expect("Error saving peer");
}

fn get_peers(conn: &SqliteConnection) -> Vec<Peer> {
    peers::table
        .load::<DbPeer>(conn)
        .expect("Error loading peers")
        .into_iter()
        .map(DbPeer::to_peer)
        .collect()
}

fn get_db_node(conn: &SqliteConnection) -> Option<DbNode> {
    nodes::table
        .load::<DbNode>(conn)
        .expect("Error loading nodes")
        .first()
        .cloned()
}

fn get_blockchain_link(conn: &SqliteConnection, start_id: i32) -> Option<DbBlockchain> {
    blockchains::table
        .filter(blockchains::head_block.eq(start_id))
        .load::<DbBlockchain>(conn)
        .expect("Error getting blockchain link")
        .first()
        .cloned()
}

fn get_block(conn: &SqliteConnection, block_id: i32) -> DbBlock {
    blocks::table
        .filter(blocks::id.eq(block_id))
        .load::<DbBlock>(conn)
        .expect("Error loading block")
        .first()
        .cloned()
        .expect("Block not found")
}

fn db_block_to_block(block: &DbBlock) -> Block<ContentTypeImpl> {
    Block::new(
        block.content.parse().expect("Could not load content of block"),
        block.prev_hash.to_string(),
        block.nonce.to_string(),
    )
}

fn get_blockchain(conn: &SqliteConnection, start_id: i32, hash_starting_pattern: String) -> Blockchain<ContentTypeImpl> {
    let mut blocks = vec![];
    let mut current_id = start_id;

    while let Some(block_link) = get_blockchain_link(conn, current_id) {
        let tail = block_link.tail_block;
        let current_block = get_block(conn, current_id);
        blocks.push(db_block_to_block(&current_block));

        current_id = tail;
    }

    let current_block = get_block(conn, current_id);
    blocks.push(db_block_to_block(&current_block));

    Blockchain::from_blocks(hash_starting_pattern, blocks)
}

pub fn load_node(conn: &diesel::SqliteConnection) -> Option<Node<ContentTypeImpl>> {
    let n = get_db_node(conn)?;
    let url = n.url.to_string();
    let start_id = n.start_block_id;
    let hash_starting_pattern = n.hash_starting_pattern.to_string();
    let blockchain = get_blockchain(conn, start_id, hash_starting_pattern);
    let peers = get_peers(conn);

    Some(Node::load(url, peers, blockchain))
}
