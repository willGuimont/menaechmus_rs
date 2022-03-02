use std::time::Instant;

use serde::Serialize;

use menaechmus::{Block, Blockchain};

fn mine_block<T: Serialize>(content: T, prev_hash: &str, hash_starting_pattern: &str) -> Block<T> {
    let mut j = 0;
    loop {
        let nonce = format!("{}", j);
        if Block::is_valid_nonce(&content, prev_hash, &nonce, hash_starting_pattern) {
            return Block::new(content, prev_hash, nonce, hash_starting_pattern).unwrap();
        }
        j += 1;
    }
}

fn main() {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let genesis_prev_hash = "";
    let mut blockchain = Blockchain::new(mine_block(0i32, genesis_prev_hash, &hash_starting_pattern));
    println!("Starting mining with difficulty {}", difficulty);
    let begin_time = Instant::now();

    for i in 1..10 {
        let prev_hash = blockchain.last_block().hash();
        let block = mine_block(i, prev_hash, &hash_starting_pattern);
        blockchain.add_block(block).unwrap();
    }
    println!("Finished mining {} blocks", blockchain.blocks().len());
    println!("Duration {:?}", begin_time.elapsed());

    println!("{:?}", blockchain);
}
