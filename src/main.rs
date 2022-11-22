use hyper::{body, Client, Uri};
use tokio::task::JoinSet;

use crate::server::start_server;

mod server;

const URL: &str = "http://127.0.0.1";

type AsyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type AsyncVoidResult = AsyncResult<()>;

#[tokio::main]
async fn main() -> AsyncVoidResult {
    start_server().await?;

    let mut responses = download_pages(10).await;
    loop {
        match responses.join_next().await {
            Some(response) => println!("{}", response??),
            None => break,
        }
    }

    Ok(())
}

async fn download_pages(n: u32) -> JoinSet<AsyncResult<String>> {
    let mut set = JoinSet::new();

    for _ in 1..=n {
        set.spawn(download_page());
    }

    set
}

async fn download_page() -> AsyncResult<String> {
    let response = Client::new().get(Uri::from_static(URL)).await?;
    let body = body::to_bytes(response.into_body()).await?;

    Ok(String::from_utf8(Vec::from(body))?)
}
