use std::time::Instant;

use menaechmus::{Block, Blockchain, ContentType};

fn mine_block<T: ContentType>(content: T, blockchain: &Blockchain<T>) -> Block<T> {
    let mut j = 0;
    loop {
        let nonce = format!("{}", j);
        let prev_hash = blockchain.last_block().hash();
        if Block::is_valid_nonce(&content, prev_hash, &nonce, blockchain.hash_starting_pattern()) {
            return Block::new(content, prev_hash.to_string(), nonce);
        }
        j += 1;
    }
}

// TODO connect to node
fn main() {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let mut blockchain = Blockchain::new(Block::new(0, "".to_string(), "".to_string()), hash_starting_pattern);
    println!("Starting mining with difficulty {}", difficulty);
    let begin_time = Instant::now();

    for i in 1..10 {
        let block = mine_block(i, &blockchain);
        blockchain.add_block(block).unwrap();
    }
    println!("Finished mining {} blocks", blockchain.blocks().len());
    println!("Duration {:?}", begin_time.elapsed());

    blockchain.blocks().iter().for_each(|b| println!("{:?}", b));
}
