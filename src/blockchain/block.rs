use bincode::serialize;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub trait ContentType = Serialize + Clone;

#[derive(Clone, Debug)]
pub struct Block<T: ContentType> {
    content: T,
    prev_hash: String,
    nonce: String,
    hash: String,
}

impl<T: ContentType> Block<T> {
    pub fn new(content: T, prev_hash: String, nonce: String) -> Block<T> {
        let hash = Self::compute_hash(&content, &prev_hash, &nonce);
        Block { content, prev_hash, nonce, hash }
    }

    pub fn is_valid(&self, hash_starting_pattern: &str) -> bool {
        Self::is_valid_nonce(&self.content, &self.prev_hash, &self.nonce, hash_starting_pattern)
    }

    pub fn is_valid_nonce(content: &T, prev_hash: &str, nonce: &String, hash_starting_pattern: &str) -> bool {
        let hash = Self::compute_hash(content, prev_hash, nonce);
        hash.starts_with(hash_starting_pattern)
    }

    fn compute_hash(content: &T, prev_hash: &str, nonce: &String) -> String {
        let mut hasher = Sha256::new();

        hasher.update(serialize(&content).unwrap());
        hasher.update(serialize(&prev_hash).unwrap());
        hasher.update(serialize(&nonce).unwrap());

        format!("{:x}", hasher.finalize())
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn prev_hash(&self) -> &String {
        &self.prev_hash
    }

    pub fn nonce(&self) -> &String {
        &self.nonce
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }
}
