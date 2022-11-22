use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::task::JoinHandle;

use crate::AsyncResult;

pub async fn start_server() -> AsyncResult<JoinHandle<()>> {
    let page_id = Arc::new(AtomicUsize::new(1));
    let page_id_generator = move || page_id.fetch_add(1, Ordering::Relaxed);

    let handle_func = make_service_fn(move |_socket| {
        let page_id_generator = page_id_generator.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |_: Request<Body>| {
                let page_id_generator = page_id_generator.clone();

                async move {
                    Ok::<_, Infallible>(Response::new(Body::from(page_id_generator().to_string())))
                }
            }))
        }
    });

    Ok(tokio::task::spawn(async move {
        let address = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), 80);

        if let Err(err) = Server::bind(&address).serve(handle_func).await {
            println!("Error serving connection: {:?}", err);
        }
    }))
}
