// FIXME: remove this feature gate as soon as the rustc version used in docs.rs is updated
#![cfg_attr(
    tsukuyomi_server_inject_extern_prelude,
    feature(extern_prelude)
)]

//! A general purpose HTTP server based on Hyper and tower-service.

#![doc(html_root_url = "https://docs.rs/tsukuyomi-server/0.1.2")]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]
#![cfg_attr(tsukuyomi_server_deny_warnings, deny(warnings))]
#![cfg_attr(
    tsukuyomi_server_deny_warnings,
    doc(test(attr(deny(warnings))))
)]

extern crate bytes;
#[macro_use]
#[doc(hidden)]
pub extern crate futures;
#[doc(hidden)]
pub extern crate http;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tower_service;

#[cfg(feature = "tls")]
#[doc(hidden)]
pub extern crate rustls;
#[cfg(feature = "tls")]
extern crate tokio_rustls;

pub mod local;
pub mod rt;
pub mod server;
pub mod service;

pub mod vendor {
    pub extern crate futures;
    pub extern crate http;
    #[cfg(feature = "tls")]
    pub extern crate rustls;
}

/// A type alias representing a critical error.
pub type CritError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn server<S>(new_service: S) -> server::Server<S> {
    server::Server::new(new_service)
}
