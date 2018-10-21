extern crate tsukuyomi_server;

mod common;

use common::Echo;
use std::net::SocketAddr;

fn main() {
    tsukuyomi_server::server(Echo::default())
        .transport(SocketAddr::from(([127, 0, 0, 1], 4000)))
        .run_forever()
        .expect("failed to run the server");
}
