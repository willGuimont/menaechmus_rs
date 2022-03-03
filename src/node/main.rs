#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::status;

mod node;

#[get("/")]
fn index() -> &'static str {
    "Menaechmus node"
}

#[get("/")]
fn health() -> &'static str {
    todo!("Need to drop peer if peer doesn't answer");
    "Node is OK"
}

#[post("/")]
fn add_peers() -> Result<String, status::BadRequest<String>> {
    // TODO a node should make a request to this route
    // TODO add peers to local peers, and update state
    // TODO broadcast local state to all peers if there was at least a new peer
    // TODO return local state
    // TODO on conflict, take the state that is returned by the most nodes
    unimplemented!("Add peer, send current status of blockchain, receive json, return badrequest on error");
}

#[post("/mine")]
fn add_mined_block() -> Result<String, status::BadRequest<String>> {
    // TODO add a new block to the chain
    // TODO validate (see blockchain.rs)
    // TODO broadcast to other nodes
    unimplemented!("Add block to chain, receive json, return badrequest on error");
}

#[get("/prompt")]
fn get_prompt() -> &'static str {
    // TODO return mining prompt
    unimplemented!("return block that is not yet mined")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![add_peers])
        .mount("/blocks", routes![add_mined_block, get_prompt])
        .launch();
}
