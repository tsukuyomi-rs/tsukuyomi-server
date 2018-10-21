use std::io;

use tsukuyomi_server::futures::{future, Async, Poll};
use tsukuyomi_server::http;
use tsukuyomi_server::http::{Request, Response};
use tsukuyomi_server::service::http::{Body, RequestBody};
use tsukuyomi_server::service::{NewService, Service};

#[derive(Default, Copy, Clone)]
pub struct Echo {
    _priv: (),
}

impl NewService for Echo {
    type Request = Request<RequestBody>;
    type Response = Response<Body>;
    type Error = io::Error;
    type Service = Echo;
    type InitError = io::Error;
    type Future = future::FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(*self)
    }
}

impl Service for Echo {
    type Request = Request<RequestBody>;
    type Response = Response<Body>;
    type Error = io::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, _request: http::Request<RequestBody>) -> Self::Future {
        future::ok(
            Response::builder()
                .header("content-type", "text/plain; charset=utf-8")
                .body("Hello, Tsukuyomi.\n".into())
                .expect("should be a valid response"),
        )
    }
}
