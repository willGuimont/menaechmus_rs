#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};

use futures::executor::block_on;
use rocket::State;
use rocket_contrib::json::Json;

use menaechmus::{Block, Blockchain, BlockchainError};

use crate::dtos::{BlockchainDto, FromDto, MinedBlockDto, PeerDto, ToDto};
use crate::node::{MiningPrompt, Node, Peer};

mod node;
mod dtos;

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
fn get_peers(node_state: State<NodeState>) -> Json<Vec<PeerDto>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let peers = node.peers().iter().map(|p| p.to_dto()).collect();
    Json(peers)
}

/// Adds new peers to the node
#[post("/", data = "<peer>")]
fn add_peer(node_state: State<NodeState>, peer: Json<Vec<Peer>>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    node.add_peers(peer.0);
    block_on(node.prune_peers());
    block_on(node.broadcast_peers());
}

#[post("/prune")]
fn prune_peers(node_state: State<NodeState>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    block_on(node.prune_peers());
}

#[get("/")]
fn get_blockchain(node_state: State<NodeState>) -> Json<BlockchainDto<ContentType>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let blockchain = node.blockchain().to_dto();
    Json(blockchain)
}

#[post("/", data = "<blockchain>")]
fn sync_blockchain(node_state: State<NodeState>, blockchain: Json<BlockchainDto<ContentType>>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let blockchain = blockchain.0.to_domain();
    node.sync_blockchain(blockchain);
}

#[post("/mine", data = "<block>")]
fn add_mined_block(node_state: State<NodeState>, block: Json<MinedBlockDto<ContentType>>) -> Result<(), BlockchainError> {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let block = block.to_domain();
    node.add_mined_block(block)?;
    block_on(node.broadcast_blockchain());
    Ok(())
}

#[get("/prompt")]
fn get_mining_prompt(node_state: State<NodeState>) -> Json<Option<MiningPrompt<ContentType>>> {
    let node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    Json(node.mining_prompt())
}

fn main() {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    let node = Node::new(blockchain);
    let node_state = Arc::new(Mutex::new(node));

    rocket::ignite()
        .manage(node_state)
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peer, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt])
        .launch();
}
