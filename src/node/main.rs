#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use clap::Parser;

use menaechmus::{Block, Blockchain};

use crate::node::{Node, Peer};
use crate::routes::*;

mod node;
mod dtos;
mod routes;

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
    let args = Args::parse();

    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    let mut node = Node::new(args.url.to_string(), blockchain, Duration::from_millis(args.timeout_ms));
    node.add_peer(Peer::new(args.url));

    if !args.peer.is_empty() {
        node.add_peer(Peer::new(args.peer));
        node.broadcast_peers().await;
    }
    let node_state = NodeState::new(node);

    let figment = rocket::Config::figment()
        .merge(("port", args.port));

    rocket::custom(figment)
        .manage(node_state)
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peers, broadcast_peers, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt, set_content])
        .launch()
        .await
        .unwrap();
}
