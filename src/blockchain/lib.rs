#![feature(trait_alias)]
extern crate serde_derive;

pub use block::*;
pub use blockchain::*;

mod block;
mod blockchain;

