/*!
Provides the handle_client function to manage client connections.

This function is responsible for reading data from the client and sending responses back.
*/
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Handles the client connection.
/// 
/// This function reads data from the client and echoes it back.
/// It runs in a loop until the client disconnects.
pub async fn handle_client(
    mut socket: TcpStream, 
    _permit: tokio::sync::OwnedSemaphorePermit
) -> Result<(), std::io::Error> {
    println!("Handling client connection");

    // This buffer is used to read data from the client.
    // The size of the buffer can be adjusted based on the expected message size.
    let mut buf = [0u8; 1024];

    // Separate the socket into read and write halves
    // This allows us to count the time for idle timeout separately for reading
    // let (mut reader, mut writer) = socket.into_split();

    // Set the allowed times for reading
    let mut retry_count = 0;
    loop {
        match socket.read(buf.as_mut()).await {
            // Dont forget to handle the case when client disconnect
            // or the socket will not release the resource
            Ok(0) => {
                println!("Client disconnected");
                return Ok(());
            },
            Ok(n) => {
                println!("Read {} bytes from client", n);

                // Echo the message back to the client
                // write the message back independently
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    return Result::Err(e);
                }
                buf = [0u8; 1024];
            },
            Err(_) if retry_count < 3 => {
                println!("Error reading from client, Retrying...");
                retry_count += 1;
                continue;
            }
            Err(e) => {
                return Result::Err(e);
            }
        }
    }
}