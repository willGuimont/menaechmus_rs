use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use menaechmus::{Block, ContentType};

use crate::Peer;

pub trait ToDto {
    type Output;
    fn to_dto(&self) -> Self::Output;
}

pub trait FromDto<'a> {
    type Output;
    fn to_domain(&'a self) -> Self::Output;
}

#[derive(Serialize)]
pub struct BlockDtoOutput<T: ContentType> {
    content: T,
    prev_hash: String,
    nonce: String,
    hash: String,
}

#[derive(Deserialize)]
#[serde(bound = "T: ContentType + for<'a> Deserialize<'a>")]
pub struct BlockDtoInput<T> where T: ContentType + for<'a> Deserialize<'a> {
    content: T,
    prev_hash: String,
    nonce: String,
}

pub struct BlockchainDto {}

pub struct MiningPromptDto<T: ContentType> {
    content: T,
    prev_hash: String,
}

impl<T: ContentType> ToDto for Block<T> {
    type Output = BlockDtoOutput<T>;

    fn to_dto(&self) -> Self::Output {
        BlockDtoOutput {
            content: self.content().clone(),
            prev_hash: self.prev_hash().clone(),
            nonce: self.nonce().clone(),
            hash: self.hash().clone(),
        }
    }
}

impl<'a, T: ContentType + DeserializeOwned> FromDto<'a> for BlockDtoInput<T> {
    type Output = Block<T>;

    fn to_domain(&'a self) -> Self::Output {
        todo!()
    }
}
