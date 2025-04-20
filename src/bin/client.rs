use echo_lib::client::{Client, handle};
use tokio::{
    net::TcpStream,
    sync::mpsc,
    // select,
    // io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() {
    let mut client = Client::new(7210, "127.0.0.1", 60).await;
    client.tcp_stream = Some(
        match client.tcp_stream {
            Some(stream) => stream,
            None => {
                TcpStream::connect(format!("{}:{}", client.host, client.port)).await.unwrap()
            }
        },
    );

    let (reader, writer) = client.tcp_stream.unwrap().into_split();
    let (tx, rx) = mpsc::channel::<String>(100);

    // This task handles user input and sends it to the server.
    let input_handle = handle::user_input_handle(tx);

    // This task handles the network communication with the server.
    // have bug with the server_stream_handle if spawn the task in the function
    // but it work well if not sealed as a function
    // fixed by take the spawn part out of the function

    // did not work
    // let network_handle = handle::server_stream_handle(
    //     rx,
    //     writer,
    //     reader,
    // );

    // works
    let network_handle = tokio::spawn(
        handle::server_stream_handle(rx, writer, reader)
    );

    // works
    // let network_handle = tokio::spawn(async move {
    //     let mut server_response = [0u8; 1024];

    //     loop {
    //         select! {
    //             // Receive messages from the user input task
    //             msg = rx.recv() => {
    //                 if let Some(msg) = msg {
    //                     if let Err(e) = writer.write_all(msg.as_bytes()).await {
    //                         eprintln!("Failed to write message: {}", e);
    //                         break;
    //                     }
    //                 }
    //             }

    //             // Read messages from the server
    //             result = reader.read(&mut server_response) => {
    //                 match result {
    //                     Ok(0) => {
    //                         println!("Connection closed");
    //                         break;
    //                     }
    //                     Ok(_) => {
    //                         println!("Server: {}", String::from_utf8_lossy(&server_response));
    //                         // Clear the buffer for the next read
    //                         server_response = [0u8; 1024];
    //                     }
    //                     Err(e) => {
    //                         eprintln!("Read error: {}", e);
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // });

    // Waiting for the tasks to finish
    let _ = tokio::join!(input_handle, network_handle);
}