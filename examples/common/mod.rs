use tsukuyomi_server::service::http::{Body, RequestBody};
use tsukuyomi_server::service::{NewService, Service};
use tsukuyomi_server::vendor::futures::{future, Async, Poll};
use tsukuyomi_server::vendor::http::{Request, Response};
use tsukuyomi_server::CritError;

#[derive(Default, Copy, Clone)]
pub struct Echo {
    _priv: (),
}

impl NewService for Echo {
    type Request = Request<RequestBody>;
    type Response = Response<Body>;
    type Error = CritError;
    type Service = Echo;
    type InitError = CritError;
    type Future = future::FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(*self)
    }
}

impl Service for Echo {
    type Request = Request<RequestBody>;
    type Response = Response<Body>;
    type Error = CritError;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, _request: Request<RequestBody>) -> Self::Future {
        future::ok(
            Response::builder()
                .header("content-type", "text/plain; charset=utf-8")
                .body("Hello, Tsukuyomi.\n".into())
                .expect("should be a valid response"),
        )
    }
}
