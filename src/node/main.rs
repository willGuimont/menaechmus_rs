#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
#[macro_use]
extern crate rocket;

use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use rocket::futures::lock::Mutex;
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

    #[clap(long, default_value_t = 500)]
    timeout_ms: u64,
}

type ContentType = i32;

struct NodeState(Arc<Mutex<Node<ContentType>>>);

// TODO document routes
/// Index route
#[get("/")]
fn index() -> &'static str {
    "Menaechmus node"
}

/// Returns health status
#[get("/")]
fn health() -> &'static str {
    "Node is OK"
}

/// Returns peers
#[get("/")]
async fn get_peers(node_state: &State<NodeState>) -> Json<Vec<PeerDto>> {
    let node = node_state.inner().0.lock().await;
    let peers = node.peers().iter().map(|p| p.to_dto()).collect();
    Json(peers)
}

/// Adds a new peer to the node, will broadcast its updated peers list to other nodes
#[post("/", data = "<peers>")]
async fn add_peers(node_state: &State<NodeState>, peers: Json<Vec<PeerDto>>) {
    let peers = peers.0.iter().map(|p| p.to_domain()).collect();
    let mut node = node_state.inner().0.lock().await;
    node.add_peers(peers);
    // TODO add a broadcast peer here once node is in a database and doesn't require shared state
}

/// Sends the current peers to other nodes
#[post("/")]
async fn broadcast_peers(node_state: &State<NodeState>) {
    let node = node_state.inner().0.lock().await;
    node.broadcast_peers().await;
}

/// Removes unreachable peers
#[post("/prune")]
async fn prune_peers(node_state: &State<NodeState>) {
    let mut node = node_state.inner().0.lock().await;
    node.prune_peers().await;
}

/// Returns the current state of the blockchain
#[get("/")]
async fn get_blockchain(node_state: &State<NodeState>) -> Json<BlockchainDto<ContentType>> {
    let node = node_state.inner().0.lock().await;
    let blockchain = node.blockchain().to_dto();
    Json(blockchain)
}

/// Updates the state of the block chain from other another node
#[post("/", data = "<blockchain>")]
async fn sync_blockchain(node_state: &State<NodeState>, blockchain: Json<BlockchainDto<ContentType>>) {
    // TODO remove, and instead query other nodes and trust the majority
    let mut node = node_state.inner().0.lock().await;
    let blockchain = blockchain.0.to_domain();
    node.sync_blockchain(blockchain);
}

/// Adds a mined block to the blockchain, might return error
#[post("/mine", data = "<block>")]
async fn add_mined_block(node_state: &State<NodeState>, block: Json<MinedBlockDto<ContentType>>) -> Json<Result<(), String>> {
    let mut node = node_state.inner().0.lock().await;
    let block = block.to_domain();
    match node.add_mined_block(block) {
        Ok(_) => {}
        Err(err) => { return Json(Err(format!("{:?}", err))); }
    }
    // TODO add a broadcast blockchain here once node is in a database and doesn't require shared state
    Json(Ok(()))
}

/// Returns the mining prompt
#[get("/prompt")]
async fn get_mining_prompt(node_state: &State<NodeState>) -> Json<Option<MiningPrompt<ContentType>>> {
    let node = node_state.inner().0.lock().await;
    Json(node.mining_prompt())
}

/// Sets the content of the next to be mined block
#[post("/content", data = "<content>")]
async fn set_content(node_state: &State<NodeState>, content: Json<ContentType>) {
    // TODO remove, for testing purposes only
    let mut node = node_state.inner().0.lock().await;
    node.set_next_content(content.0);
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    let mut node = Node::new(args.url, blockchain, Duration::from_millis(args.timeout_ms));

    if args.peer != "" {
        node.add_peers(vec![Peer::new(args.peer)]);
        node.broadcast_peers().await;
    }
    let node_state = NodeState(Arc::new(Mutex::new(node)));

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
