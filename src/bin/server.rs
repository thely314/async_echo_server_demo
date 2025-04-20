use echo_lib::server::Server;

#[tokio::main]
async fn main() {
    let mut echo_server = Server::new(7210, "127.0.0.1", 1000);
    echo_server.start().await;
}