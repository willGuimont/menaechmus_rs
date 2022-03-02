use std::collections::linked_list::LinkedList;

use serde::Serialize;

use crate::Block;

#[derive(Serialize, Debug)]
pub struct Blockchain<T: Serialize> {
    blocks: LinkedList<Block<T>>,
}

#[derive(Debug, Clone)]
pub enum BlockchainError {
    BadPreviousHashError
}

impl<T: Serialize> Blockchain<T> {
    pub fn new(genesis: Block<T>) -> Blockchain<T> {
        let mut blocks = LinkedList::new();
        blocks.push_back(genesis);
        Blockchain { blocks }
    }

    pub fn add_block(&mut self, block: Block<T>) -> Result<(), BlockchainError> {
        let prev_hash = self.blocks.back().unwrap().hash();
        if block.prev_hash() == prev_hash {
            self.blocks.push_back(block);
            Ok(())
        } else {
            Err(BlockchainError::BadPreviousHashError)
        }
    }

    pub fn blocks(&self) -> &LinkedList<Block<T>> {
        &self.blocks
    }

    pub fn last_block(&self) -> &Block<T> {
        &self.blocks.back().unwrap()
    }
}
