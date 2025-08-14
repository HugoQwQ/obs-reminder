use crate::audio_manager::AudioManager;
use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, body::Bytes};
use hyper_util::rt::TokioIo;
use rust_embed::RustEmbed;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

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

    // Handle audio file requests by UUID
    if path.starts_with("/audio/") {
        return handle_audio_request(path).await;
    }

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

async fn handle_audio_request(
    path: &str,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Infallible> {
    // Extract the UUID from the URL
    // Format: /audio/uuid
    let file_id = &path[7..]; // Remove "/audio/"

    // Create audio manager and get file path
    match AudioManager::new() {
        Ok(audio_manager) => {
            if let Some(file_path) = audio_manager.get_audio_file_path(file_id) {
                match tokio::fs::read(&file_path).await {
                    Ok(content) => {
                        let mime_type = mime_guess::from_path(&file_path)
                            .first_or_octet_stream()
                            .as_ref()
                            .to_string();

                        log::info!(
                            "Serving audio file: {} (size: {} bytes, mime: {})",
                            file_path.display(),
                            content.len(),
                            mime_type
                        );

                        let body = Full::new(Bytes::from(content))
                            .map_err(|never| match never {})
                            .boxed();

                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("content-type", mime_type)
                            .header("accept-ranges", "bytes")
                            .header("cache-control", "public, max-age=3600")
                            .header("access-control-allow-origin", "*")
                            .body(body)
                            .unwrap())
                    }
                    Err(e) => {
                        log::error!("Failed to read audio file '{}': {}", file_path.display(), e);
                        let body = Full::new(Bytes::from("Failed to read audio file"))
                            .map_err(|never| match never {})
                            .boxed();

                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(body)
                            .unwrap())
                    }
                }
            } else {
                log::warn!("Audio file not found for ID: {}", file_id);
                let body = Full::new(Bytes::from("Audio file not found"))
                    .map_err(|never| match never {})
                    .boxed();

                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body)
                    .unwrap())
            }
        }
        Err(e) => {
            log::error!("Audio manager error: {}", e);
            let body = Full::new(Bytes::from("Audio manager error"))
                .map_err(|never| match never {})
                .boxed();

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}
