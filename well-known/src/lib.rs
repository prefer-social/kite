//! web services under url .well-known {webfineger, host-meta}  
//!
//!

use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use crate::hostmeta::hostmeta;
use crate::webfinger::webfinger;

pub mod hostmeta;
pub mod webfinger;

#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();
    tracing::debug!(
        "<---------- ({request_method}) {request_path_and_query} --------->"
    );

    let mut router = Router::new();

    router.get_async("/.well-known/webfinger", webfinger);
    router.get_async("/.well-known/host-meta", hostmeta);

    router.handle_async(req).await
}

pub async fn get_http_headers_map(req: &Request) {
    let headers = req.headers();
    for header in headers {
        println!("{header:?}");
    }
}
