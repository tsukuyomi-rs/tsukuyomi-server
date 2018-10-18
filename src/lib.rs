//! A general purpose HTTP server based on Hyper and tower-service.

#![doc(html_root_url = "https://docs.rs/tsukuyomi-server/0.1.0")]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    unused
)]

extern crate bytes;
#[macro_use]
pub extern crate futures;
pub extern crate http;
extern crate hyper;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tower_service;

#[cfg(feature = "tls")]
pub extern crate rustls;
#[cfg(feature = "tls")]
extern crate tokio_rustls;

pub mod local;
pub mod rt;
pub mod server;
pub mod service;

/// A type alias representing a critical error.
pub type CritError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn server<S>(new_service: S) -> crate::server::Server<S> {
    crate::server::Server::new(new_service)
}
