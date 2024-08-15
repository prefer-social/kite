use anyhow::{Error, Result};
use reqwest as request;
use serde_json;
use url::Url;

use sparrow::webfinger::WebFinger;

pub async fn search(given_uri: &String) {
    // -> Result<String> {
    let uri = match given_uri.starts_with("@") {
        true => given_uri[1..].to_string(),
        false => given_uri.to_owned(),
    };
    let url_string = format!("uri://{}", uri);
    let uri_url = Url::parse(url_string.as_str()).unwrap();

    let a = format!(
        "https://{}/.well-known/webfinger?resource=acct:{}@{}",
        uri_url.domain().unwrap(),
        uri_url.username(),
        uri_url.domain().unwrap(),
    );

    tracing::debug!(a);

    let wf = call_webfinger(a.as_str()).await;

    //let actor_url = get_actor_url(wf);

    //Ok(uri)
}

fn get_actor_url(wf: WebFinger) {
    //-> Result<Url> {

    tracing::error!("0000000000000");
    let links = wf.links.iter().filter(|x| x.rel == "self");
    tracing::debug!("---->{:?}", links);
}

async fn call_webfinger(webfinger_url: &str) {
    // -> Result<WebFinger> {
    tracing::debug!(webfinger_url);
    let body = reqwest::get(webfinger_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap(); //.text().await;
                   //let wf = match body {
                   //    Ok(x) => serde_json::from_str::<WebFinger>(&x).unwrap(),
                   //    Err(e) => panic!("{:?}", e),
                   //};

    tracing::debug!("{:?}", body);

    // tracing::debug!("{}", body.unwrap());

    // match serde_json::from_str::<WebFinger>(&body.unwrap()) {
    //     Ok(o) => Ok(o),
    //     Err(e) => Err(Error::msg(format!("{e:?}"))),
    // }
    //Ok(wf)
}

//#[tokio::test]
// async fn search_test() {
//     assert_eq!(
//         search(&"foo@bar.com".to_string()).await,
//         "foo@bar.com".to_string()
//     );
// }
