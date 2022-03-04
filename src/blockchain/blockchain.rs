use std::collections::linked_list::LinkedList;

use serde::Serialize;

use crate::Block;

#[derive(Serialize, Debug, Clone)]
pub struct Blockchain<T: Serialize> {
    hash_starting_pattern: String,
    blocks: LinkedList<Block<T>>,
}

#[derive(Debug, Clone)]
pub enum BlockchainError {
    BadPreviousHashError,
    InvalidBlockError,
}

impl<T: Serialize> Blockchain<T> {
    pub fn new(genesis: Block<T>, hash_starting_pattern: String) -> Blockchain<T> {
        let mut blocks = LinkedList::new();
        blocks.push_back(genesis);
        Blockchain { hash_starting_pattern, blocks }
    }

    pub fn add_block(&mut self, block: Block<T>) -> Result<(), BlockchainError> {
        let prev_hash = self.blocks.back().unwrap().hash();

        if block.prev_hash() != prev_hash {
            return Err(BlockchainError::BadPreviousHashError);
        }

        if !block.is_valid(&self.hash_starting_pattern) {
            return Err(BlockchainError::InvalidBlockError);
        }

        self.blocks.push_back(block);
        Ok(())
    }

    pub fn blocks(&self) -> &LinkedList<Block<T>> {
        &self.blocks
    }

    pub fn last_block(&self) -> &Block<T> {
        &self.blocks.back().unwrap()
    }

    pub fn hash_starting_pattern(&self) -> &str {
        &self.hash_starting_pattern
    }
}
