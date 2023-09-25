mod utils;
mod proxy_handlers;
use std::{net::SocketAddr, convert::Infallible, env::{var, self}};

use hyper::service::{make_service_fn, service_fn};
use log::info;
use utils::httpsclient::HTTPSClient;

#[tokio::main]
async fn main() {
    env_logger::init();

    let environment = var("ENV").unwrap_or("Development".to_string());

    info!("{}", environment);

    let address = SocketAddr::from(([0,0,0,0], 3000));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(proxy_handlers::handle))
    });

    let server = hyper::Server::bind(&address).serve(make_service);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}