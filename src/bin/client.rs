use echo_lib::client::{Client, handle};
use tokio::{
    net::TcpStream,
    sync::mpsc,
};

#[tokio::main]
async fn main() {
    let mut client = Client::new(7210, "127.0.0.1").await;
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
    let input_handle = handle::user_input_handle(tx).await;

    // This task handles the network communication with the server.
    let network_handle = handle::server_stream_handle(
        rx,
        writer,
        reader
    ).await;

    // Waiting for the tasks to finish
    let _ = tokio::join!(input_handle, network_handle);
}