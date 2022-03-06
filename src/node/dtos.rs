use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, Blockchain, ContentType};

use crate::node::Peer;

pub trait ToDto {
    type Output;
    fn to_dto(&self) -> Self::Output;
}

pub trait FromDto {
    type Output;
    fn to_domain(&self) -> Self::Output;
}

#[derive(Serialize)]
pub struct PeerDto {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlockDto<T: ContentType> {
    content: T,
    prev_hash: String,
    nonce: String,
    hash: String,
}

#[derive(Deserialize)]
pub struct MinedBlockDto<T: ContentType> {
    content: T,
    prev_hash: String,
    nonce: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlockchainDto<T: ContentType> {
    hash_starting_pattern: String,
    blocks: Vec<BlockDto<T>>,
}

#[allow(dead_code)]
pub struct MiningPromptDto<T: ContentType> {
    content: T,
    prev_hash: String,
}

impl PeerDto {
    pub fn new(url: String) -> PeerDto {
        PeerDto { url }
    }
}

impl ToDto for Peer {
    type Output = PeerDto;

    fn to_dto(&self) -> Self::Output {
        PeerDto {
            url: self.url().clone(),
        }
    }
}

impl<T: ContentType> ToDto for Block<T> {
    type Output = BlockDto<T>;

    fn to_dto(&self) -> Self::Output {
        BlockDto {
            content: self.content().clone(),
            prev_hash: self.prev_hash().clone(),
            nonce: self.nonce().clone(),
            hash: self.hash().clone(),
        }
    }
}

impl<T: ContentType> FromDto for BlockDto<T> {
    type Output = Block<T>;

    fn to_domain(&self) -> Self::Output {
        Block::new(
            self.content.clone(),
            self.prev_hash.to_string(),
            self.nonce.to_string())
    }
}

impl<T: ContentType> FromDto for MinedBlockDto<T> {
    type Output = Block<T>;

    fn to_domain(&self) -> Self::Output {
        Block::new(self.content.clone(), self.prev_hash.clone(), self.nonce.clone())
    }
}

impl<T: ContentType> ToDto for Blockchain<T> {
    type Output = BlockchainDto<T>;

    fn to_dto(&self) -> Self::Output {
        BlockchainDto {
            hash_starting_pattern: self.hash_starting_pattern().to_string(),
            blocks: self.blocks().iter().map(|b| b.to_dto()).collect(),
        }
    }
}

impl<T: ContentType> FromDto for BlockchainDto<T> {
    type Output = Blockchain<T>;

    fn to_domain(&self) -> Self::Output {
        Blockchain::from_blocks(
            self.hash_starting_pattern.clone(),
            self.blocks.iter().map(|b| b.to_domain()).collect())
    }
}
