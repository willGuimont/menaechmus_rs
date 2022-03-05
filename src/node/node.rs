use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, Blockchain, ContentType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Peer {
    url: String,
}

pub struct Node<T: ContentType> {
    peers: Vec<Peer>,
    blockchain: Blockchain<T>,
}

#[derive(Serialize)]
pub struct MiningPrompt<T: ContentType> {
    content: T,
    prev_hash: String,
}

impl<T: ContentType> Node<T> {
    pub fn new(blockchain: Blockchain<T>) -> Node<T> {
        Node { peers: vec![], blockchain }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        unimplemented!();
    }

    pub fn broadcast_peers(&self) {
        unimplemented!();
    }

    pub fn broadcast_mined_block(&self) {
        unimplemented!();
    }

    pub fn broadcast_blockchain(&self) {
        unimplemented!();
    }

    pub fn add_mined_block(&mut self, block: Block<T>) {
        unimplemented!();
    }

    pub fn mining_prompt(&self) -> MiningPrompt<T> {
        unimplemented!();
    }

    pub fn peers(&self) -> Vec<Peer> {
        self.peers.clone()
    }

    pub fn blockchain(&self) -> Blockchain<T> {
        unimplemented!()
    }
}
