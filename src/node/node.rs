use serde::Serialize;

use menaechmus::Blockchain;

pub struct Peer {
    url: String,
}

pub struct Node<T: Serialize> {
    peers: Vec<Peer>,
    blockchain: Blockchain<T>,
}

impl<T: Serialize> Node<T> {

}
