use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitystream::actor::person::Person;
<<<<<<< HEAD
=======
use sparrow::mastodon::create_get_signrature;
use sparrow::utils::get_current_time_in_rfc_1123;
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

<<<<<<< HEAD
    let (account, _user) =
        sparrow::mastodon::account::Account::default().await?;
    let actor = Person::new(account).await.unwrap();
    let s = serde_json::to_string(&actor).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"")
=======
    let (account, _user) = sparrow::mastodon::account::Account::default().await?;
    let actor = Person::new(account).await.unwrap();
    let s = serde_json::to_string(&actor).unwrap();

    let date = get_current_time_in_rfc_1123().await;

    let signature =
        create_get_signrature(sender_actor_url, sender_private_key_pem, request_url, date);

    Ok(Response::builder()
        .status(200)
        .header(
            "Content-Type",
            "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
        )
        .header("Date", date)
        .header("Signature", &signature)
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        .body(s)
        .build())
}
