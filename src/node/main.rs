#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use clap::Parser;
use rocket::{Build, Rocket};

use menaechmus::{Block, Blockchain};

use crate::models::{DbConn, load_node, update_node};
use crate::node::{ContentTypeImpl, Node, Peer};
use crate::routes::*;

mod node;
mod dtos;
mod routes;
mod models;
mod schema;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[clap(long, default_value_t = 8000)]
    port: u16,

    #[clap(long, default_value = "")]
    peer: String,

    #[clap(long)]
    url: String,

    #[clap(long, default_value_t = 500)]
    timeout_ms: u64,
}

async fn get_node(rock: &Rocket<Build>, node_url: String, peer_url: String) -> Node<ContentTypeImpl> {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);

    let conn = DbConn::get_one(&rock).await.expect("Could not connect to database");
    let mut node = conn.run(|c| load_node(c))
        .await
        .or_else(|| Some(Node::new(node_url.to_string(), blockchain)))
        .unwrap();

    if peer_url != "" {
        node.add_peers(vec![Peer::new(peer_url)]);
        node.broadcast_peers(Duration::from_millis(500)).await;
    }

    node
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    let figment = rocket::Config::figment()
        .merge(("port", args.port));

    let rock = rocket::custom(figment)
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peers, broadcast_peers, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt, set_content])
        .launch()
        .await
        .unwrap();
}
