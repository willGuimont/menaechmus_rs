#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use clap::Parser;

use menaechmus::{Block, Blockchain};

use crate::models::DbConn;
use crate::node::{Node, Peer};
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

#[rocket::main]
async fn main() {
    // TODO logic to store node starting configuration
    let args = Args::parse();
    let node_config = NodeConfig {
        timeout: Duration::from_millis(args.timeout_ms),
        url: args.url,
    };

    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    let mut node = Node::new(node_config.url.to_string(), blockchain);

    if args.peer != "" {
        node.add_peers(vec![Peer::new(args.peer)]);
        node.broadcast_peers(node_config.timeout).await;
    }
    // TODO save node to db

    let figment = rocket::Config::figment()
        .merge(("port", args.port));

    rocket::custom(figment)
        .manage(node_config)
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peers, broadcast_peers, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt, set_content])
        .launch()
        .await
        .unwrap();
}
