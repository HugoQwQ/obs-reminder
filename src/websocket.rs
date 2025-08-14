use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

pub const WEBSOCKET_PORT: u16 = 7981;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastMessage {
    pub title: String,
    pub content: String,
    pub color_1: String,
    pub color_2: String,
    pub text_color: String,
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String,
    pub data: ToastMessage,
}

impl WebSocketMessage {
    pub fn new_toast(title: String, content: String, color_1: String, color_2: String, text_color: String, duration_seconds: u32) -> Self {
        Self {
            r#type: "toast".to_string(),
            data: ToastMessage {
                title,
                content,
                color_1,
                color_2,
                text_color,
                duration: duration_seconds * 1000, // Convert to milliseconds
            },
        }
    }
    
    pub fn to_message(&self) -> Result<Message, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(Message::Text(json))
    }
}

pub struct WebSocketServer {
    addr: SocketAddr,
    sender: broadcast::Sender<WebSocketMessage>,
    client_count: Arc<Mutex<usize>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(100);
        let addr = SocketAddr::from(([127, 0, 0, 1], WEBSOCKET_PORT));
        
        Self {
            addr,
            sender,
            client_count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn get_sender(&self) -> broadcast::Sender<WebSocketMessage> {
        self.sender.clone()
    }
    
    pub async fn _get_client_count(&self) -> usize {
        *self.client_count.lock().await
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(&self.addr).await?;
        log::info!("WebSocket server listening on {}", self.addr);
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    log::info!("New WebSocket connection from {}", addr);
                    
                    let sender = self.sender.clone();
                    let receiver = self.sender.subscribe();
                    let client_count = self.client_count.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, sender, receiver, client_count).await {
                            log::error!("Error handling WebSocket connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
    
    pub async fn _broadcast(&self, message: WebSocketMessage) -> Result<(), broadcast::error::SendError<WebSocketMessage>> {
        log::debug!("Broadcasting message: {:?}", message);
        self.sender.send(message)?;
        Ok(())
    }
}

async fn handle_connection(
    stream: TcpStream,
    _sender: broadcast::Sender<WebSocketMessage>,
    mut receiver: broadcast::Receiver<WebSocketMessage>,
    client_count: Arc<Mutex<usize>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    log::info!("WebSocket connection established");
    
    // Increment client count
    {
        let mut count = client_count.lock().await;
        *count += 1;
        log::info!("Client connected. Total clients: {}", *count);
    }
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    // Handle incoming messages from client (if any)
    let client_count_clone = client_count.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    log::debug!("Received message from client: {}", text);
                }
                Ok(Message::Close(_)) => {
                    log::info!("Client sent close message");
                    break;
                }
                Err(e) => {
                    log::error!("Error receiving message: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        // Decrement client count when connection closes
        let mut count = client_count_clone.lock().await;
        *count -= 1;
        log::info!("Client disconnected. Total clients: {}", *count);
    });
    
    // Handle outgoing messages to client
    let send_task = tokio::spawn(async move {
        while let Ok(message) = receiver.recv().await {
            match message.to_message() {
                Ok(ws_message) => {
                    if let Err(e) = ws_sender.send(ws_message).await {
                        log::error!("Failed to send message to client: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("Failed to serialize message: {}", e);
                }
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = receive_task => {
            log::debug!("Receive task completed");
        }
        _ = send_task => {
            log::debug!("Send task completed");
        }
    }
    
    Ok(())
}