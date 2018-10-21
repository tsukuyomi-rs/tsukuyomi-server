# `tsukuyomi-server`

[![crates.io](https://img.shields.io/crates/v/tsukuyomi-server.svg)](https://crates.io/crates/tsukuyomi-server)
[![Docs.rs](https://docs.rs/tsukuyomi-server/badge.svg)](https://docs.rs/tsukuyomi-server)
[![Master Doc](https://img.shields.io/badge/doc-master-blue.svg)](https://tsukuyomi-rs.github.io/tsukuyomi-server)
[![dependency status](https://deps.rs/crate/tsukuyomi-server/0.1.1/status.svg)](https://deps.rs/crate/tsukuyomi-server/0.1.1)
[![Build Status](https://travis-ci.org/tsukuyomi-rs/tsukuyomi-server.svg?branch=master)](https://travis-ci.org/tsukuyomi-rs/tsukuyomi-server)

A general purpose HTTP server based on Hyper and tower-service.

## Usage 

```rust,no_run
extern crate futures;
extern crate http;
extern crate tower_service;
extern crate tsukuyomi_server;

fn main() {
    let new_service = {
        use tsukuyomi_server::service::http::{RequestBody, Body};

        struct Echo;
        impl tower_service::Service for Echo {
            type Request = http::Request<RequestBody>;
            type Response = http::Response<Body>;
            type Error = tsukuyomi_server::CritError;
            type Future = futures::future::FutureResult<Self::Response, Self::Error>;

            fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                Ok(futures::Async::Ready(()))
            }

            fn call(&mut self, request: Self::Request) -> Self::Future {
                futures::future::ok(
                    http::Response::builder()
                        .body("Hello".into())
                        .expect("should be a valid response")
                )
            }
        }

        || Ok::<_, std::io::Error>(Echo)
    };

    tsukuyomi_server::server(new_service)
        .transport(std::net::SocketAddr::from(([127, 0, 0, 1], 4000)))
        .run_forever()
        .expect("failed to start the server");
}
```

## License

[MIT license](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.
