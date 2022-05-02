extern crate rocket;

use rocket::serde::json::Json;
use menaechmus::{Block, Blockchain};

use crate::dtos::{BlockchainDto, MinedBlockDto, PeerDto, ToDto};
use crate::models::{DbConn, load_node};
use crate::node::{ContentTypeImpl, MiningPrompt, Node};
use crate::{Peer, update_node};


async fn get_node_with_peer(conn: DbConn, peers: Option<Peer>) -> Node<ContentTypeImpl> {
    let node = conn.run(|c| load_node(c)).await;
    if node.is_none() {
        // TODO get from peers
        let difficulty = 3;
        let hash_starting_pattern = "0".repeat(difficulty);
        let blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
        let mut node = Node::new("aaa".to_string(), blockchain);

        conn.run(|c| update_node(c, node)).await;
    }
    conn.run(|c| load_node(c)).await.unwrap()
}

async fn get_node(conn: DbConn) -> Node<ContentTypeImpl> {
    get_node_with_peer(conn, None).await
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
pub async fn get_peers(conn: DbConn) -> Json<Vec<PeerDto>> {
    let node = get_node(conn).await;
    // let node = load_node(&conn).unwrap();
    let peers = node.peers().iter()
        .map(|p| p.to_dto())
        .collect();
    Json(peers)
}

/// Adds a new peer to the node, will broadcast its updated peers list to other nodes
#[post("/", data = "<peers>")]
pub async fn add_peers(peers: Json<Vec<PeerDto>>) {
    // let peers = peers.0.iter().map(|p| p.to_domain()).collect();
    // let mut node = node_config.inner().0.lock().await;
    // node.add_peers(peers);
    // TODO add a broadcast peer here once node is in a database and doesn't require shared state
    unimplemented!()
}

/// Sends the current peers to other nodes
#[post("/broadcast")]
pub async fn broadcast_peers() {
    // let node = node_config.inner().0.lock().await;
    // node.broadcast_peers().await;
    unimplemented!()
}

/// Removes unreachable peers
#[post("/prune")]
pub async fn prune_peers() {
    // let mut node = node_config.inner().0.lock().await;
    // node.prune_peers().await;
    unimplemented!()
}

/// Returns the current state of the blockchain
#[get("/")]
pub async fn get_blockchain() -> Json<BlockchainDto<ContentTypeImpl>> {
    // let node = node_config.inner().0.lock().await;
    // let blockchain = node.blockchain().to_dto();
    // Json(blockchain)
    unimplemented!()
}

/// Updates the state of the block chain from other another node
#[post("/", data = "<blockchain>")]
pub async fn sync_blockchain(blockchain: Json<BlockchainDto<ContentTypeImpl>>) {
    // TODO remove, and instead query other nodes and trust the majority
    // let mut node = node_config.inner().0.lock().await;
    // let blockchain = blockchain.0.to_domain();
    // node.sync_blockchain(blockchain);
    unimplemented!()
}

/// Adds a mined block to the blockchain, might return error
#[post("/mine", data = "<block>")]
pub async fn add_mined_block(block: Json<MinedBlockDto<ContentTypeImpl>>) -> Json<Result<(), String>> {
    // let mut node = node_config.inner().0.lock().await;
    // let block = block.to_domain();
    // match node.add_mined_block(block) {
    //     Ok(_) => {}
    //     Err(err) => { return Json(Err(format!("{:?}", err))); }
    // }
    // TODO add a broadcast blockchain here once node is in a database and doesn't require shared state
    // Json(Ok(()))
    unimplemented!()
}

/// Returns the mining prompt
#[get("/prompt")]
pub async fn get_mining_prompt() -> Json<Option<MiningPrompt<ContentTypeImpl>>> {
    // let node = node_config.inner().0.lock().await;
    // Json(node.mining_prompt())
    unimplemented!()
}

/// Sets the content of the next to be mined block
#[post("/content", data = "<content>")]
pub async fn set_content(content: Json<ContentTypeImpl>) {
    // TODO remove, for testing purposes only
    // let mut node = node_config.inner().0.lock().await;
    // node.set_next_content(content.0);
    unimplemented!()
}
