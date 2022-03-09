#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};

use clap::Parser;
use rocket::serde::json::Json;
use rocket::State;

use menaechmus::{Block, Blockchain};

use crate::dtos::{BlockchainDto, FromDto, MinedBlockDto, PeerDto, ToDto};
use crate::node::{MiningPrompt, Node, Peer};

mod node;
mod dtos;

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
}

type ContentType = i32;

struct NodeState(Arc<Mutex<Node<ContentType>>>);

// TODO document routes
#[get("/")]
fn index() -> &'static str {
    "Menaechmus node"
}

/// Return node status
#[get("/")]
fn health() -> &'static str {
    "Node is OK"
}

#[get("/")]
fn get_peers(node_state: &State<NodeState>) -> Json<Vec<PeerDto>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let peers = node.peers().iter().map(|p| p.to_dto()).collect();
    Json(peers)
}

#[post("/", data = "<peers>")]
async fn add_peers(node_state: &State<NodeState>, peers: Json<Vec<PeerDto>>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let peers = peers.0.iter().map(|p| p.to_domain()).collect();
    node.add_peers(peers);
    node.broadcast_peers();
}

#[post("/prune")]
fn prune_peers(node_state: &State<NodeState>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    node.prune_peers();
}

#[get("/")]
fn get_blockchain(node_state: &State<NodeState>) -> Json<BlockchainDto<ContentType>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let blockchain = node.blockchain().to_dto();
    Json(blockchain)
}

#[post("/", data = "<blockchain>")]
fn sync_blockchain(node_state: &State<NodeState>, blockchain: Json<BlockchainDto<ContentType>>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let blockchain = blockchain.0.to_domain();
    node.sync_blockchain(blockchain);
}

#[post("/mine", data = "<block>")]
fn add_mined_block(node_state: &State<NodeState>, block: Json<MinedBlockDto<ContentType>>) -> Json<Result<(), String>> {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let block = block.to_domain();
    match node.add_mined_block(block) {
        Ok(_) => {}
        Err(err) => { return Json(Err(format!("{:?}", err))); }
    }
    node.broadcast_blockchain();
    Json(Ok(()))
}

#[get("/prompt")]
fn get_mining_prompt(node_state: &State<NodeState>) -> Json<Option<MiningPrompt<ContentType>>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    Json(node.mining_prompt())
}

#[post("/content", data = "<content>")]
fn set_content(node_state: &State<NodeState>, content: Json<ContentType>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    node.set_next_content(content.0);
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    let mut node = Node::new(args.url, blockchain);

    if args.peer != "" {
        node.add_peers(vec![Peer::new(args.peer)]);
        node.broadcast_peers().await.unwrap();
    }
    let node_state = NodeState(Arc::new(Mutex::new(node)));

    let figment = rocket::Config::figment()
        .merge(("port", args.port));

    rocket::custom(figment)
        .manage(node_state)
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peers, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt, set_content])
        .launch()
        .await
        .unwrap();
}
