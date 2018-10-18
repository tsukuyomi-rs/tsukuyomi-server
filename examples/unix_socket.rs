extern crate tsukuyomi_server;

mod common;

fn main() {
    imp::main()
}

#[cfg(unix)]
mod imp {
    use crate::common::Echo;
    use std::path::Path;

    pub fn main() {
        tsukuyomi_server::server(Echo::default())
            .transport(Path::new("/tmp/unix-socket.sock"))
            .run_forever()
            .expect("failed to run the server");
    }
}

#[cfg(not(unix))]
mod imp {
    pub fn main() {
        println!("This example works only on Unix platform.");
    }
}
