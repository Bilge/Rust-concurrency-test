use crate::AsyncResult;
use hyper::{body, Client, Uri};
use tokio::task::JoinSet;

const URL: &str = "http://127.0.0.1";

pub async fn download_pages(n: usize) -> JoinSet<AsyncResult<String>> {
    let mut set = JoinSet::new();

    for _ in 0..n {
        set.spawn(download_page());
    }

    set
}

async fn download_page() -> AsyncResult<String> {
    let response = Client::new().get(Uri::from_static(URL)).await?;
    let body = body::to_bytes(response.into_body()).await?;

    Ok(String::from_utf8(Vec::from(body))?)
}
