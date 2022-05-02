use std::collections::HashSet;
use std::time::Duration;

use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, Blockchain, BlockchainError, ContentType};

use crate::dtos::{PeerDto, ToDto};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Peer {
    url: String,
}

pub struct Node<T: ContentType> {
    url: String,
    // TODO remove url field
    next_content: Option<T>,
    peers: HashSet<Peer>,
    blockchain: Blockchain<T>,
    timeout: Duration,
}

#[derive(Serialize)]
pub struct MiningPrompt<T: ContentType> {
    content: T,
    prev_hash: String,
}

impl Peer {
    pub fn new(url: String) -> Peer {
        Peer { url }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

impl<T: ContentType> Node<T> {
    pub fn new(url: String, blockchain: Blockchain<T>, timeout: Duration) -> Node<T> {
        let peers = HashSet::new();
        Node {
            url: url.to_string(),
            next_content: None,
            peers,
            blockchain,
            timeout,
        }
    }

    pub fn add_peers(&mut self, peers: Vec<Peer>) {
        self.peers.extend(peers);
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.insert(peer);
    }

    pub async fn broadcast_peers(&self) {
        Self::broadcast_from_peers(self.url.to_string(), self.peers.clone(), &self.timeout).await;
    }

    pub async fn broadcast_from_peers(url: String, peers: HashSet<Peer>, timeout: &Duration) {
        let client = reqwest::Client::new();
        let peers_dto: Vec<PeerDto> = peers.iter().map(|p| p.to_dto()).collect();

        for p in peers.iter().filter(|p| p.url != url) {
            let _ = client.post(p.url.to_string() + "/peers")
                .json(&peers_dto)
                .timeout(*timeout)
                .send()
                .await;
        }
    }

    pub async fn prune_peers(&mut self) {
        let client = reqwest::Client::new();
        let mut new_peers = HashSet::new();
        for p in &self.peers {
            let status = client.get(p.url.to_string() + "/health")
                .timeout(self.timeout)
                .send()
                .await
                .ok()
                .map(|r| r.status());
            if let Some(StatusCode::OK) = status {
                new_peers.insert(p.clone());
            }
        }
        self.peers = new_peers;
    }

    pub async fn broadcast_blockchain(&self, timeout: Duration) {
        let client = reqwest::Client::new();
        let blockchain = self.blockchain().to_dto();

        for p in &self.peers {
            let _ = client.post(p.url.to_string() + "/blocks")
                .json(&blockchain)
                .timeout(timeout)
                .send()
                .await;
        }
    }

    pub fn sync_blockchain(&mut self, blockchain: Blockchain<T>) {
        // TODO handle if chain is different (take the most common amongst peers)
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

    // TODO remove
    #[deprecated]
    pub fn set_next_content(&mut self, content: T) {
        self.next_content = Some(content);
    }

    pub fn peers(&self) -> &HashSet<Peer> {
        &self.peers
    }

    pub fn blockchain(&self) -> &Blockchain<T> {
        &self.blockchain
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn timeout(&self) -> &Duration {
        &self.timeout
    }
}
