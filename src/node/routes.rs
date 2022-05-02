extern crate rocket;

use std::sync::Arc;

use rocket::futures::lock::Mutex;
use rocket::serde::json::Json;
use rocket::State;

use crate::dtos::{BlockchainDto, FromDto, MinedBlockDto, PeerDto, ToDto};
use crate::node::{MiningPrompt, Node};

pub type ContentTypeImpl = i32;

pub struct NodeState(Arc<Mutex<Node<ContentTypeImpl>>>);

impl NodeState {
    pub fn new(node: Node<ContentTypeImpl>) -> NodeState {
        NodeState(Arc::new(Mutex::new(node)))
    }
}

/// Index route
#[get("/")]
pub fn index() -> &'static str {
    "Menaechmus node"
}

/// Returns health status
#[get("/")]
pub fn health() -> &'static str {
    "Node is OK"
}

/// Returns peers
#[get("/")]
pub async fn get_peers(node_state: &State<NodeState>) -> Json<Vec<PeerDto>> {
    let node = node_state.inner().0.lock().await;
    let peers = node.peers().iter().map(|p| p.to_dto()).collect();
    Json(peers)
}

/// Adds a new peer to the node, will broadcast its updated peers list to other nodes
#[post("/", data = "<peers>")]
pub async fn add_peers(node_state: &State<NodeState>, peers: Json<Vec<PeerDto>>) {
    let (node_peers, timeout, url, blockchain) = {
        let peers = peers.0.iter().map(|p| p.to_domain()).collect();
        let mut node = node_state.inner().0.lock().await;
        let new_peers = node.add_peers(peers);

        if !new_peers {
            return;
        }

        (node.peers().clone(), *node.timeout(), &node.url().to_string(), node.blockchain().clone())
    };
    Node::<ContentTypeImpl>::broadcast_from_peers(url, &node_peers, &timeout).await;
    Node::<ContentTypeImpl>::broadcast_blockchain_from_peers(url, &node_peers, &timeout, &blockchain).await;
}

/// Sends the current peers to other nodes
#[post("/broadcast")]
pub async fn broadcast_peers(node_state: &State<NodeState>) {
    let node = node_state.inner().0.lock().await;
    node.broadcast_peers().await;
}

/// Removes unreachable peers
#[post("/prune")]
pub async fn prune_peers(node_state: &State<NodeState>) {
    // TODO prune peers
    // let mut node = node_state.inner().0.lock().await;
    // node.prune_peers().await;
    unimplemented!()
}

/// Returns the current state of the blockchain
#[get("/")]
pub async fn get_blockchain(node_state: &State<NodeState>) -> Json<BlockchainDto<ContentTypeImpl>> {
    let node = node_state.inner().0.lock().await;
    let blockchain = node.blockchain().to_dto();
    Json(blockchain)
}

/// Updates the state of the block chain from another node
#[post("/", data = "<blockchain>")]
pub async fn sync_blockchain(node_state: &State<NodeState>, blockchain: Json<BlockchainDto<ContentTypeImpl>>) {
    // TODO remove, and instead query other nodes and trust the majority
    let mut node = node_state.inner().0.lock().await;
    let blockchain = blockchain.0.to_domain();
    node.sync_blockchain(blockchain);
    // TODO if new, broadcast to peers
}

/// Adds a mined block to the blockchain, might return error
#[post("/mine", data = "<block>")]
pub async fn add_mined_block(node_state: &State<NodeState>, block: Json<MinedBlockDto<ContentTypeImpl>>) -> Json<Result<(), String>> {
    let (node_peers, timeout, url, blockchain) = {
        let mut node = node_state.inner().0.lock().await;
        let block = block.to_domain();
        match node.add_mined_block(block) {
            Ok(_) => {}
            Err(err) => { return Json(Err(format!("{:?}", err))); }
        }

        (node.peers().clone(), *node.timeout(), &node.url().to_string(), node.blockchain().clone())
    };

    Node::<ContentTypeImpl>::broadcast_blockchain_from_peers(url, &node_peers, &timeout, &blockchain).await;
    Json(Ok(()))
}

/// Returns the mining prompt
#[get("/prompt")]
pub async fn get_mining_prompt(node_state: &State<NodeState>) -> Json<Option<MiningPrompt<ContentTypeImpl>>> {
    let node = node_state.inner().0.lock().await;
    Json(node.mining_prompt())
}

/// Sets the content of the next to be mined block
#[post("/content", data = "<content>")]
pub async fn set_content(node_state: &State<NodeState>, content: Json<ContentTypeImpl>) {
    // TODO remove, for testing purposes only
    let mut node = node_state.inner().0.lock().await;
    node.set_next_content(content.0);
}
