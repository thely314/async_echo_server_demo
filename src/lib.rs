pub mod server;
pub mod client;

/// This is a simple echo server client benchmark tool.
/// 
/// It connects to a server, sends a payload of random bytes, and measures the time taken for the server to echo back the same payload.
use bytes::{BufMut, Bytes, BytesMut};
use rand::Rng;
use std::time::Instant;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug, Clone)]
pub struct BenchClient {
    server_addr: String,
    payload_size: usize,
}

impl BenchClient {
    pub fn new(server_addr: &str, payload_size: usize) -> Self {
        Self {
            server_addr: server_addr.to_string(),
            payload_size,
        }
    }

    /// Generate a random payload of specified size
    /// 
    /// This function creates a `Bytes` object filled with random bytes.
    /// The size of the payload is determined by the `payload_size` field.
    fn generate_payload(&self) -> Bytes {
        let mut rng = rand::thread_rng();
        let mut payload = BytesMut::with_capacity(self.payload_size);
        for _ in 0..self.payload_size {
            // payload.put_u8(rand::random::<u8>());
            payload.put_u8(rng.gen_range(0..=255));
        }
        payload.freeze()
    }

    /// Send a single request to the server and measure the latency
    /// 
    /// This function connects to the server, sends a payload of random bytes, and waits for the server to echo back the same payload.
    pub async fn single_request(&self) -> Result<u128, anyhow::Error> {
        let start = Instant::now();
        let mut stream = TcpStream::connect(&self.server_addr).await?;
        
        let payload = self.generate_payload();
        stream.write_all(&payload).await?;
        
        let mut buf = vec![0u8; self.payload_size];
        stream.read_exact(&mut buf).await?;
        
        Ok(start.elapsed().as_micros())
    }

    /// Entry point for running the benchmark
    /// 
    /// This function runs multiple concurrent requests to the server and collects the results.
    /// It uses the `tokio` runtime to spawn tasks for each request.
    pub async fn run_concurrent(
        &self,
        concurrency: usize,
    ) -> Result<BenchResult, anyhow::Error> {
        let tasks = (0..concurrency)
            .map(|_| async {
                self.single_request().await
            })
            .collect::<Vec<_>>();

        let results = futures::future::join_all(tasks).await;
        
        // Collect the results and count errors
        let mut latencies = Vec::with_capacity(concurrency);
        let mut errors = 0;

        for res in results {
            match res {
                Ok(t) => latencies.push(t),
                Err(_) => errors += 1,
            }
        }
        
        Ok(BenchResult {
            total_requests: concurrency,
            errors,
            latencies,
        })
    }
}

#[derive(Debug)]
pub struct BenchResult {
    pub total_requests: usize,
    pub errors: usize,
    pub latencies: Vec<u128>,
}