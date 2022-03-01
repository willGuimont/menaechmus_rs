use menaechmus::Block;

fn main() {
    let difficulty = 3;
    let hash_starting_pattern = "0".repeat(difficulty);
    let mut blocks: Vec<Block<usize>> = vec![];
    println!("Starting mining with difficulty {}", difficulty);

    for i in 0..10 {
        let prev_hash = if i == 0 { "".to_string() } else { blocks.get(i - 1).unwrap().hash() };
        let mut j = 0;
        'mining: loop {
            let b = Block::new(i, &prev_hash, format!("{}", j), &hash_starting_pattern);
            if let Some(b) = b {
                println!("Mined block {}", i);
                println!("{:?}", &b);
                blocks.push(b);
                break 'mining;
            }
            j += 1;
        }
    }
}