use rust_concurrency_test::client::download_pages;
use rust_concurrency_test::server::start_server;
use rust_concurrency_test::AsyncVoidResult;

#[tokio::main]
async fn main() -> AsyncVoidResult {
    start_server().await;

    let mut responses = download_pages(10).await;
    loop {
        match responses.join_next().await {
            Some(response) => println!("{}", response??),
            None => break,
        }
    }

    Ok(())
}
