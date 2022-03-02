use bincode::serialize;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize, Debug)]
pub struct Block<T: Serialize> {
    content: T,
    prev_hash: String,
    nonce: String,
    hash: String,
}

impl<T: Serialize> Block<T> {
    pub fn new(content: T, prev_hash: &str, nonce: String, hash_starting_pattern: &str) -> Option<Block<T>> {
        if Self::is_valid_nonce(&content, prev_hash, &nonce, hash_starting_pattern) {
            let prev_hash = prev_hash.to_string();
            let nonce = nonce.to_string();
            let hash = Self::compute_hash(&content, &prev_hash, &nonce);
            Some(Block { content, prev_hash, nonce, hash })
        } else {
            None
        }
    }

    pub fn is_valid_nonce(content: &T, prev_hash: &str, nonce: &String, hash_starting_pattern: &str) -> bool {
        let hash = Self::compute_hash(&content, prev_hash, &nonce);
        return hash.starts_with(hash_starting_pattern);
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
