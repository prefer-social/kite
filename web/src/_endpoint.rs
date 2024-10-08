use spin_sdk::http::{Request, Response, Router};

use crate::actor;

pub async fn router(req: Request) -> Response {
    // Accept request
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept

    // Modifying headers, Custom header (Sample)
    // let mut headers = req
    let headers = req
        .headers()
        .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
        .collect::<Vec<_>>();

    //headers.push(("foo".to_string(), "bar".as_bytes().to_vec()));

    //let modified_uri = "http://0.0.0.0:8003/foo";
    let req = Request::builder()
        //.uri(modified_uri)
        .uri(req.uri())
        .method(req.method().clone())
        .headers(headers.clone())
        .body(req.into_body())
        .build();

    let _path = req.path_and_query().unwrap().to_string();
    let owner =
        sparrow::mastodon::setting::Setting::get("site_contact_username")
            .await
            .unwrap();

    let mut router = Router::new();

    // Actor endpoints (multiple for compatibility)
    router.any_async("/", actor::req);
    router.any_async("/self", actor::req);
    router.any_async(format!("@{}", owner).as_str(), actor::req);
    // Just to compatible with Mastodon
    router.any_async(format!("/users/{}", owner).as_str(), actor::req);

    router.handle_async(req).await

    // }
}
