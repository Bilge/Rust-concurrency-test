use rust_concurrency_test::client::download_pages;
use rust_concurrency_test::server::start_server;

#[tokio::test]
async fn main() {
    const EXPECTED: [&str; 10] = ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10"];

    let mut response_count = 0;

    start_server().await;

    let mut responses = download_pages(EXPECTED.len()).await;
    loop {
        match responses.join_next().await {
            Some(response) => {
                response_count += 1;

                let response = response.unwrap().unwrap();

                assert!(
                    EXPECTED.contains(&&*response),
                    "\"{response}\" not in {:#?}.",
                    EXPECTED
                );
            }
            None => break,
        }
    }

    assert_eq!(
        EXPECTED.len(),
        response_count,
        "{response_count} not equal to {}.",
        EXPECTED.len()
    );
}
