use std::time::Instant;
use std::env;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: stress_test_cli <url> <num_requests> <concurrency>");
        return;
    }

    let url = &args[1];
    let num_requests: u32 = args[2].parse().unwrap();
    let concurrency: u32 = args[3].parse().unwrap();

    let client = Client::new();
    let mut handles = vec![];

    let start = Instant::now();

    for _ in 0..concurrency {
        let client = client.clone();
        let url = url.clone();

        let handle = tokio::spawn(async move {
            for _ in 0..(num_requests / concurrency) {
                let _ = client.get(&url).send().await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let requests_per_second = num_requests as f64 / elapsed;

    println!("Total requests: {}", num_requests);
    println!("Concurrency level: {}", concurrency);
    println!("Elapsed time: {:.2} seconds", elapsed);
    println!("Requests per second: {:.2}", requests_per_second);
}