use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, Blockchain, BlockchainError, ContentType};

use crate::dtos::{PeerDto, ToDto};

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

    // TODO remove Result output
    pub async fn broadcast_peers(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let peers: Vec<PeerDto> = self.peers.iter().map(|p| p.to_dto()).collect();

        for p in &self.peers {
            client.post(p.url.to_string() + "/peers")
                .json(&peers)
                .send()
                .await?;
        }
        Ok(())
    }

    // TODO remove Result output
    pub async fn prune_peers(&mut self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let mut new_peers = vec![];
        for p in &self.peers {
            let status = client.get(p.url.to_string() + "/health")
                .send()
                .await?
                .status();
            if status == StatusCode::OK {
                new_peers.push(p.clone());
            }
        }
        self.peers = new_peers;
        Ok(())
    }

    // TODO remove Result output
    pub async fn broadcast_blockchain(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let blockchain = self.blockchain().to_dto();

        for p in &self.peers {
            client.post(p.url.to_string() + "/blocks")
                .json(&blockchain)
                .send()
                .await?;
        }
        Ok(())
    }

    pub fn sync_blockchain(&mut self, blockchain: Blockchain<T>) {
        // TODO handle if chain is different (take the most common amongs peers)
        self.blockchain = blockchain;
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
