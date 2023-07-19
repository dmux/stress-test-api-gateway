# stress-test-api-gateway

## Description

Here's an example of a simple command-line interface (CLI) for stress testing an Generic API Gateway using Rust and the reqwest library.

## Usage

1. Clone the repository
2. Run `cargo build --release`
3. Run `./target/release/stress-test-api-gateway`
4. Enter the API Gateway URL
5. Enter the number of requests to send
6. Enter the number of concurrent requests to send

## Example

```shell
$ ./target/release/stress-test-api-gateway http://localhost:8000 1000 2
Total requests: 1000
Concurrency level: 2
Elapsed time: 0.60 seconds
Requests per second: 1660.90

```
