use std::convert::Infallible;
use hyper::{Body, Request, Response};
use log::info;

pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let route = _req.uri();
    info!("{}", route);
    Ok(Response::new(Body::from("Hello World")))
}
