use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

use menaechmus::Blockchain;

#[derive(Serialize, Deserialize, Clone)]
pub struct Peer {
    url: String,
}

#[derive(Serialize)]
pub struct Node<T: Serialize> {
    peers: Vec<Peer>,
    blockchain: Blockchain<T>,
}

#[derive(Serialize)]
pub struct MiningPrompt {}

impl<T: Serialize> Node<T> {
    pub fn new(blockchain: Blockchain<T>) -> Node<T> {
        Node { peers: vec![], blockchain }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        unimplemented!();
    }

    pub fn broadcast_to_peers(&self) {
        unimplemented!();
    }

    pub fn add_mined_block(&mut self) {
        unimplemented!();
    }

    pub fn mining_prompt(&self) -> MiningPrompt {
        unimplemented!();
    }

    pub fn peers(&self) -> Vec<Peer> {
        self.peers.clone()
    }

    pub fn blockchain(&self) -> Blockchain<T> {
        self.blockchain.clone()
    }
}
