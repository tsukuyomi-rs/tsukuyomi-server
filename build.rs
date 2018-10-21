use std::env;

fn main() {
    if env::var_os("TSUKUYOMI_SERVER_DENY_WARNINGS").is_some() {
        println!("cargo:rustc-cfg=tsukuyomi_server_deny_warnings");
    }
}
