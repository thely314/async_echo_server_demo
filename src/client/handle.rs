/*!
Provides the user_input_handle and server_stream_handle functions to manage user input and server communication.

These functions are responsible for reading user input from the standard input and sending it to the server, as well as reading messages from the server and writing them to the standard output.
These functions run in separate tasks to allow for concurrent execution.
*/
use Result::Ok;
use tokio::{
    io::{BufReader, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    select
};

/// Handles user input from the standard input.
/// 
/// This function creates a task that reads lines from the standard input and sends them to the provided channel.
/// It runs in a loop until the channel is closed or an error occurs.
pub async fn user_input_handle(tx: tokio::sync::mpsc::Sender<String>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while tx.capacity() == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }

        let stdin = BufReader::new(tokio::io::stdin());
        let mut lines = stdin.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if tx.send(line).await.is_err() {
                eprintln!("Failed to send message to network task");
                break;
            }
        }
    })
}

/// Handles the server stream.
/// 
/// This function creates a task that reads messages from the server and writes them to the standard output.
/// It also reads messages from the user input task and sends them to the server.
pub async fn server_stream_handle(
    mut rx: tokio::sync::mpsc::Receiver<String>, 
    mut writer: OwnedWriteHalf, 
    mut reader: OwnedReadHalf
// ) -> tokio::task::JoinHandle<()> {
) -> Result<(), anyhow::Error> {
    // tokio::spawn(async move {
        let mut server_response = [0u8; 1024];

        loop {
            select! {
                // Receive messages from the user input task
                msg = rx.recv() => {
                    if let Some(msg) = msg {
                        if let Err(e) = writer.write_all(msg.as_bytes()).await {
                            eprintln!("Failed to write message: {}", e);
                            break anyhow::Ok(());
                        }
                    }
                }

                // Read messages from the server
                result = reader.read(&mut server_response) => {
                    match result {
                        Ok(0) => {
                            println!("Connection closed");
                            break anyhow::Ok(());
                        }
                        Ok(_) => {
                            println!("Server: {}", String::from_utf8_lossy(&server_response));
                            // Clear the buffer for the next read
                            server_response = [0u8; 1024];
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break anyhow::Ok(());
                        }
                    }
                }
            }
        }
    // })
}