use serde::Serialize;
use serde_derive::Serialize;

use menaechmus::Blockchain;

pub struct Peer {
    url: String,
}

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

    pub fn add_peers(&mut self, peers: Vec<Peer>) {
        unimplemented!();
    }

    pub fn broadcast_to_peers() {
        unimplemented!();
    }

    pub fn add_mined_block() {
        unimplemented!();
    }

    pub fn mining_prompt(&self) -> MiningPrompt {
        unimplemented!();
    }
}
