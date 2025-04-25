/*!
This module provides the Client struct and its associated methods for connecting to a server.

It includes methods to create a new client instance and establish a connection to the server.
The Client struct contains the server's host and port information, as well as a timeout value.
*/
pub mod handle;
use tokio::net::TcpStream;

pub struct Client {
    // target server port
    pub port: u16,
    // target server host
    pub host: String,

    pub tcp_stream: Option<TcpStream>,
}

impl Client {
    /// Create a new Client instance.
    /// 
    /// This function initializes the client with the specified port, host, and timeout values.
    pub async fn new(port: u16, host_str: &str) -> Self {
        let tcp_stream = TcpStream::connect(format!("{}:{}", host_str, port));
        match tcp_stream.await {
            Ok(stream) => {
                println!("Connected to server at {}:{}", host_str, port);
                Client {
                    port,
                    host: host_str.to_string(),
                    tcp_stream: Some(stream),
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
                Client {
                    port,
                    host: host_str.to_string(),
                    tcp_stream: None,
                }
            }
        }
    }
}