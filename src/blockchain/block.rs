use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block<T> {
    content: T,
    prev_hash: String,
    nonce: String,
}

impl<T: ToString> Block<T> {
    pub fn new(content: T, prev_hash: &String, nonce: String, difficulty: usize) -> Option<Block<T>> {
        let prev_hash = prev_hash.to_string();

        let block = Block { content, prev_hash, nonce };
        let hash = block.hash();
        if hash.starts_with(&"0".repeat(difficulty.clone())) {
            Some(block)
        } else {
            None
        }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(&self.content.to_string());
        hasher.update(&self.prev_hash.as_bytes());
        hasher.update(&self.nonce.as_bytes());

        format!("{:x}", hasher.finalize())
    }
}
