use hyper::{body, Client, Uri};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::server::start_server;

mod server;

const URL: &str = "http://127.0.0.1";

type AsyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type AsyncVoidResult = AsyncResult<()>;

#[tokio::main]
async fn main() -> AsyncVoidResult {
    start_server().await?;

    let mut responses = download_pages(10).await?;
    loop {
        match responses.recv().await {
            Some(response) => println!("{response}"),
            None => break,
        }
    }

    Ok(())
}

async fn download_pages(n: u32) -> AsyncResult<Receiver<String>> {
    let (tx, rx) = mpsc::channel(1);

    for _ in 1..=n {
        tokio::task::spawn(download_page(tx.clone()));
    }

    Ok(rx)
}

async fn download_page(tx: Sender<String>) -> AsyncVoidResult {
    let response = Client::new().get(Uri::from_static(URL)).await?;
    let body = body::to_bytes(response.into_body()).await?;

    tx.send(String::from_utf8(Vec::from(body))?).await?;

    Ok(())
}
