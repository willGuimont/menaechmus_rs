#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
extern crate core;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use clap::Parser;
use rocket::{Build, Rocket};

use menaechmus::{Block, Blockchain};

use crate::node::{ContentTypeImpl, Node, Peer};
use crate::routes::*;

mod node;
mod dtos;
mod routes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[clap(long, default_value_t = 8000)]
    port: u16,

    #[clap(long, default_value = "")]
    peer: String,

    #[clap(long)]
    url: String,

    #[clap(long, default_value_t = 500)]
    timeout_ms: u64,
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    let figment = rocket::Config::figment()
        .merge(("port", args.port));

    let rock = rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/health", routes![health])
        .mount("/peers", routes![get_peers, add_peers, broadcast_peers, prune_peers])
        .mount("/blocks", routes![get_blockchain, sync_blockchain, add_mined_block, get_mining_prompt, set_content])
        .launch()
        .await
        .unwrap();
}
