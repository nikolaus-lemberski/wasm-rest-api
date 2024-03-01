use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(text_response(String::from("Hello"))),

        (&Method::GET, "/goodbye") => Ok(text_response(String::from("Goodbye"))),

        _ => Ok(not_found()),
    }
}

fn text_response(text: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(text))
        .unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_svc = make_service_fn(|_| async move {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req)))
    });
    let server = Server::bind(&addr).serve(make_svc);
    dbg!("Server started on port 8080");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
