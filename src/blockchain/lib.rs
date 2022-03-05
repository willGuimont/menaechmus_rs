#![feature(trait_alias)]
#[macro_use]
extern crate serde_derive;

pub use block::*;
pub use blockchain::*;

mod block;
mod blockchain;

