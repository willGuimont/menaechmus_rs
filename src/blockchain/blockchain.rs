use std::collections::LinkedList;

use crate::Block;

struct Blockchain<T> {
    blocks: LinkedList<Block<T>>,
}

#[derive(Debug, Clone)]
enum BlockchainError {
    BadPreviousHashError
}

impl<T> Blockchain<T> {
    pub fn new() {
        Blockchain(blocks = LinkedList::new())
    }

    pub fn add_block(&self, block: Block<T>) -> Result<Blockchain<T>, BlockchainError> {
        unimplemented!("Add block to the chain");
    }
}
