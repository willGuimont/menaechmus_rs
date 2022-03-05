#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};

use rocket::State;
use rocket_contrib::json::Json;

use menaechmus::{Block, Blockchain};
use crate::dtos::{BlockDtoInput, BlockDtoOutput, FromDto};

use crate::node::{MiningPrompt, Node, Peer};
use crate::dtos::ToDto;

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
    todo!("Need to drop peer if peer doesn't answer");
    "Node is OK"
}

/// Adds a new peer to the node
///     If the peer is new to the node, it will broadcast it to its peers
#[post("/", data = "<peer>")]
fn add_peer(node_state: State<NodeState>, peer: Json<Peer>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    node.add_peer(peer.0);
    node.broadcast_peers();
}

#[get("/")]
fn get_peers(node_state: State<NodeState>) -> Json<Vec<Peer>> {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let peers = node.peers();
    Json(peers)
}

#[post("/mine", data = "<block>")]
fn add_mined_block(node_state: State<NodeState>, block: BlockDtoInput<ContentType>) {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    let block = block.to_domain();
    node.add_mined_block(block);
    node.broadcast_mined_block();
}

#[get("/prompt")]
fn get_mining_prompt(node_state: State<NodeState>) -> Json<MiningPrompt<ContentType>> {
    let mut node = node_state.inner().0.lock().expect("Failed to acquire lock on state");
    Json(node.mining_prompt())
}

fn main() {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let mut blockchain = Blockchain::new(Block::new(0, "", "".to_string()), hash_starting_pattern);
    let mut node = Node::new(blockchain);
    let mut node_state = Arc::new(Mutex::new(node));

    rocket::ignite()
        .manage(node_state)
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![add_peer, get_peers])
        .mount("/blocks", routes![add_mined_block, get_mining_prompt])
        .launch();
}
