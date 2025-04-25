/*!
This module provides the Server struct and its methods to manage a TCP server.

It includes methods to start the server, accept incoming connections, and handle client requests.
The server listens on a specified host and port, and can handle multiple connections concurrently.
*/
pub mod handle;
use tokio::{
    task::spawn,
    net::TcpListener,
    sync::Semaphore,
};
use std::sync::Arc;

pub struct Server {
    // target listen port
    pub port: u16,
    // target listen host
    pub host: String,

    // TODO Have not implemented the max_connections configuration yet
    // control the max connections
    pub max_connections: Arc<Semaphore>,

    tcp_listener: Option<TcpListener>,
}

impl Server {
    /// Create a new Server instance.
    /// 
    /// This function initializes the server with the specified port, host, maximum connections, and timeout values.
    pub fn new(port: u16, host_str: &str, max_connections: usize) -> Self {
        Server {
            port,
            host: host_str.to_string(),
            max_connections: Arc::new(Semaphore::new(max_connections)),
            tcp_listener: None,
        }
    }

    /// Start the server and listen for incoming connections.
    /// 
    /// This method will block the current thread until the server is stopped.
    /// It will spawn a new task for each incoming connection to handle client requests.
    pub async fn start(&mut self) {
        println!("Starting server on {}:{}", self.host, self.port);
        self.tcp_listener = Some(
            match TcpListener::bind(format!("{}:{}", self.host, self.port)).await {
                Ok(listener) => {
                    println!("Server started successfully");
                    listener
                }
                Err(e) => {
                    eprintln!("Failed to start server: {}", e);
                    return;
                }
            },
        );

        let tcp_listener = self.tcp_listener.as_mut().unwrap();

        loop {
            // Waiting for a permit to be available
            let permit = match self.max_connections.clone().acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("Max connections reached, waiting for a permit...");
                    continue;
                }
            };

            // Wating for incoming connections asynchronously
            let (socket, _) = match tcp_listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                },
            };

            spawn(async move {
                println!("Accepted connection from {}", socket.peer_addr().unwrap());
                match handle::handle_client(socket, permit).await {
                    Ok(_) => println!("Handle ended"),
                    Err(e) => eprintln!("Error handling client: {}", e),
                }
            });
        }
    }
}

/// implement Drop trait for Server
/// This trait is used to clean up resources when the Server instance is dropped.
impl Drop for Server {
    fn drop(&mut self) {
        println!("Server is shutting down");
        if let Some(listener) = self.tcp_listener.take() {
            drop(listener);
        }
    }
}