# `tsukuyomi-server`

[![crates.io](https://img.shields.io/crates/v/tsukuyomi-server.svg)](https://crates.io/crates/tsukuyomi-server)
[![Docs.rs](https://docs.rs/tsukuyomi-server/badge.svg)](https://docs.rs/tsukuyomi-server)
[![Master Doc](https://img.shields.io/badge/doc-master-blue.svg)](https://tsukuyomi-rs.github.io/tsukuyomi-server)
[![dependency status](https://deps.rs/crate/tsukuyomi-server/0.1.2/status.svg)](https://deps.rs/crate/tsukuyomi-server/0.1.2)
[![Build Status](https://travis-ci.org/tsukuyomi-rs/tsukuyomi-server.svg?branch=master)](https://travis-ci.org/tsukuyomi-rs/tsukuyomi-server)

A general purpose HTTP server based on Hyper and tower-service.

## Usage 

```rust,no_run
extern crate tsukuyomi_server;

use std::net::SocketAddr;

fn main() {
    let new_service = {
        use tsukuyomi_server::CritError;
        use tsukuyomi_server::service::Service;
        use tsukuyomi_server::service::http::{RequestBody, Body};
        use tsukuyomi_server::vendor::futures::{future, Poll, Async};
        use tsukuyomi_server::vendor::http::{Request, Response};

        struct Echo;
        impl Service for Echo {
            type Request = Request<RequestBody>;
            type Response = Response<Body>;
            type Error = CritError;
            type Future = future::FutureResult<Self::Response, Self::Error>;

            fn poll_ready(&mut self) -> Poll<(), Self::Error> {
                Ok(Async::Ready(()))
            }

            fn call(&mut self, _request: Self::Request) -> Self::Future {
                future::ok(
                    Response::builder()
                        .body("Hello".into())
                        .expect("should be a valid response")
                )
            }
        }

        || Ok::<_, CritError>(Echo)
    };

    tsukuyomi_server::server(new_service)
        .transport(SocketAddr::from(([127, 0, 0, 1], 4000)))
        .run_forever()
        .expect("failed to start the server");
}
```

## License

[MIT license](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.
