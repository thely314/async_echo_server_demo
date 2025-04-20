/// This file is part of the Echo Server Benchmarking project.
/// 
/// It is a benchmarking tool for the Echo Server implemented in Rust.
/// Use criterion for benchmarking.
use criterion::{criterion_group, criterion_main, Criterion};
use echo_lib::BenchClient;
use tokio::runtime::Runtime;

fn bench_echo(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = BenchClient::new("127.0.0.1:7210", 1024); // 1KB payload
    
    let mut group = c.benchmark_group("echo_server");

    // Test at different concurrency levels
    for concurrency in [10, 100, 1000].iter() {
        group.bench_with_input(
            format!("concurrent_{}", concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on(async {
                        client.run_concurrent(concurrency).await.unwrap()
                    })
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_echo);
criterion_main!(benches);