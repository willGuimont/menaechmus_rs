use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, Blockchain, BlockchainError, ContentType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Peer {
    url: String,
}

pub struct Node<T: ContentType> {
    next_content: Option<T>,
    peers: Vec<Peer>,
    blockchain: Blockchain<T>,
}

#[derive(Serialize)]
pub struct MiningPrompt<T: ContentType> {
    content: T,
    prev_hash: String,
}

impl Peer {
    pub fn url(&self) -> &String {
        &self.url
    }
}

impl<T: ContentType> Node<T> {
    pub fn new(blockchain: Blockchain<T>) -> Node<T> {
        Node { next_content: None, peers: vec![], blockchain }
    }

    pub fn add_peers(&mut self, peers: Vec<Peer>) {
        self.peers.extend(peers);
    }

    pub fn broadcast_peers(&self) {
        // TODO remove peers if they don't answer
        todo!("Broadcast all peers to peers")
    }

    pub fn broadcast_mined_block(&self) {
        // TODO remove peers if they don't answer
        todo!("Broadcast a newly mined block to peers")
    }

    pub fn add_mined_block(&mut self, block: Block<T>) -> Result<(), BlockchainError> {
        self.blockchain.add_block(block)
    }

    pub fn mining_prompt(&self) -> Option<MiningPrompt<T>> {
        self.next_content.clone().map(|content| {
            MiningPrompt {
                content,
                prev_hash: self.blockchain.last_block().hash().clone(),
            }
        })
    }

    pub fn peers(&self) -> &Vec<Peer> {
        &self.peers
    }

    pub fn blockchain(&self) -> &Blockchain<T> {
        &self.blockchain
    }
}
