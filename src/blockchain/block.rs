use bincode::serialize;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize, Debug)]
pub struct Block<T> {
    content: T,
    prev_hash: String,
    nonce: String,
}

impl<T: Serialize> Block<T> {
    pub fn new(content: T, prev_hash: &String, nonce: String, hash_starting_pattern: &str) -> Option<Block<T>> {
        let hash = Self::hash_helper(&content, prev_hash, &nonce);
        if hash.starts_with(hash_starting_pattern) {
            let prev_hash = prev_hash.to_string();
            Some(Block { content, prev_hash, nonce })
        } else {
            None
        }
    }

    pub fn hash(&self) -> String {
        Self::hash_helper(&self.content, &self.prev_hash, &self.nonce)
    }

    fn hash_helper(content: &T, prev_hash: &String, nonce: &String) -> String {
        let mut hasher = Sha256::new();

        hasher.update(serialize(&content).unwrap());
        hasher.update(serialize(&prev_hash).unwrap());
        hasher.update(serialize(&nonce).unwrap());

        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::blockchain::Block;

    use self::test::Bencher;

    #[bench]
    fn bench_mining_blocks(b: &mut Bencher) {
        b.iter(|| {
            let difficulty = 4;
            let hash_starting_pattern = "0".repeat(difficulty);
            let mut blocks: Vec<Block<usize>> = vec![];

            for i in 0..10 {
                let prev_hash = if i == 0 { "".to_string() } else { blocks.get(i - 1).unwrap().hash() };
                let mut j = 0;
                'mining: loop {
                    let b = Block::new(i, &prev_hash, format!("{}", j), &hash_starting_pattern);
                    if let Some(b) = b {
                        blocks.push(b);
                        break 'mining;
                    }
                    j += 1;
                }
            }
        });
    }

    fn test_mining() {}
}
