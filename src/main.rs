mod echo;
mod manual;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::error::Error> {
    let mut response = Response::new(Body::empty());

    if req.uri().path() == "/" {
        *response.body_mut() = Body::from("Try POSTing data to /echo");
        Ok(response)
    } else if req.uri().path().starts_with("/echo") {
        echo::echo_server(req).await
    } else if req.uri().path().starts_with("/manual") {
        manual::manual_server(req).await
    } else {
        *response.status_mut() = StatusCode::NOT_FOUND;
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 12345));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}