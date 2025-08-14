use rust_embed::RustEmbed;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Bytes, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use tokio::net::TcpListener;
use std::convert::Infallible;
use std::net::SocketAddr;

#[derive(RustEmbed)]
#[folder = "./browser/build/"]
struct Assets;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct HttpServer {
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> Result<(), BoxError> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;
        
        log::info!("HTTP server listening on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
                {
                    log::error!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Infallible> {
    let path = req.uri().path();
    
    // Remove leading slash and handle root path
    let file_path = if path == "/" {
        "index.html"
    } else {
        &path[1..] // Remove leading slash
    };

    match Assets::get(file_path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(file_path)
                .first_or_octet_stream()
                .as_ref()
                .to_string();

            let body = Full::new(Bytes::copy_from_slice(&content.data))
                .map_err(|never| match never {})
                .boxed();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", mime_type)
                .body(body)
                .unwrap())
        }
        None => {
            // For SPA routing, fallback to index.html
            if let Some(content) = Assets::get("index.html") {
                let body = Full::new(Bytes::copy_from_slice(&content.data))
                    .map_err(|never| match never {})
                    .boxed();

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "text/html")
                    .body(body)
                    .unwrap())
            } else {
                let body = Full::new(Bytes::from("Not Found"))
                    .map_err(|never| match never {})
                    .boxed();

                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body)
                    .unwrap())
            }
        }
    }
}