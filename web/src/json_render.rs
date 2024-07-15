use spin_sdk::http::{Params, Request, Response, Router};

pub mod followers;
pub mod following;
pub mod inbox;
pub mod outbox;

pub async fn renderer(req: Request) -> Response {
    // Accept request
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept

    // Modifying headers, Custom header (Sample)
    //let mut headers = req
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

    // End of modifying headers

    // Get Admin / Main /

    //let _domain = req.header("host").unwrap().as_str().unwrap();
    //let _domain_from_env = variables::get("DOMAIN").unwrap();

    let path = req.path_and_query().unwrap().to_string();
    let mut router = Router::new();

    router.any_async("/inbox", inbox::req);
    router.any_async("/outbox", outbox::req);
    router.any_async("/followers", followers::req);
    router.any_async("/following", following::req);

    router.handle_async(req).await

    // match path.starts_with("/@") {
    //     true => endpoint::actor::request(req, Params::new()).await.unwrap(),
    //     false => {
    //         let mut router = Router::new();

    //         router.any_async("/", endpoint::actor::request);
    //         router.get_async("/u/:user", endpoint::actor::request);
    //         router.get_async("/users/:user", endpoint::actor::request);
    //         //router.get_async("/actor", endpoint::actor::request);
    //         router.get_async("/self", endpoint::actor::request);

    //         //router.any_async("/inbox", endpoint::inbox::request);

    //         //router.any_async("/outbox", endpoint::outbox::request);

    //         router.any_async("/following", endpoint::following::request);
    //         router.any_async("/followers", endpoint::followers::request);
    //         // router.any_async("/users/:user/following", users::following::request);

    //         // router.any_async("/users/:user/followers", users::followers::request);
    //         // router.any_async("/users/:user/collections/featured", featured::request);
    //         //router.get_async("/users/:user/collections/tags", tags::request);

    //         // tests
    //         //router.any_async("/tests/db", tests::db);

    //         //
    //         //router.any_async("/foo/following_request", foo::following_request);
    //         //router.any_async("/foo", foo::modified_header_test);
    //         //router.any_async("/", foo::root);
    //         router.handle_async(req).await
    //     }
    // }
}

// foo, for debug purpose
async fn foo(_req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("foo foo")
        .build())
}
