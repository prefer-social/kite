use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use std::collections::HashMap;
use std::str;
use url::Url;

use sparrow::mastodon::application::Application;
use sparrow::mastodon::username::Username;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

// GET /oauth/authorize
// https://docs.joinmastodon.org/methods/oauth/#authorize
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    // Displays an authorization form to the user. If approved, it will create and return an authorization code, then redirect to the desired redirect_uri, or show the authorization code if urn:ietf:wg:oauth:2.0:oob was requested. The authorization code can be used while requesting a token to obtain access to user-level methods.
    let mut login_failed = "".to_string();
    let query = req.uri();
    // /oauth/authorize?response_type=code&client_id=client_id&redirect_uri=icecubesapp://&scope=read%20write%20follow%20push
    // /oauth/authorize?auth_failed
    let r = Url::parse(query).unwrap();
    let hash_query: HashMap<_, _> = r.query_pairs().into_owned().collect();
    if hash_query.get("auth_failed").is_some() {
        login_failed = "Login Failed".to_string();
    }

    let body = str::from_utf8(req.body()).unwrap();
    let a: HashMap<&str, &str> =
        querystring::querify(body).into_iter().collect();
    tracing::debug!("hashmap --> {a:?}");

    // response_type, client_id and redirect_uri are mandatory fileds.
    // it there is missing,
    // return this: https://docs.joinmastodon.org/methods/oauth/#400-client-error
    let client_id = match hash_query.get("client_id") {
        Some(x) => x,
        None => {
            return auth_client_error();
        }
    };
    let redirect_uri = match hash_query.get("redirect_uri") {
        Some(x) => x,
        None => {
            return auth_client_error();
        }
    };
    let response_type = match hash_query.get("response_type") {
        Some(x) => x,
        None => {
            return auth_client_error();
        }
    };
    let empty = &"".to_string();
    let scope = hash_query.get("scope").unwrap_or(empty);

    let simple_login = format!(
        r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login Page</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
            text-align: center;
            margin: 0;
            padding: 0;
        }}

        .login-box {{
            width: 300px;
            margin: 150px auto;
            padding: 20px;
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }}

        .login-box input {{
            width: 100%;
            padding: 8px;
            margin-bottom: 10px;
            box-sizing: border-box;
        }}

        .login-box button {{
            width: 100%;
            padding: 10px;
            background-color: #3498db;
            color: #fff;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }}
    </style>
</head>
<body>
    {login_failed}
    <div class="login-box">
        <h2>Login</h2>
        <form action="/oauth/authorize" method="POST">
            <input type="text" name="Username" required>
            <br>
            <input type="password" name="Password" required>
            <br>
            <button type="submit">Login</button>
            <input type="hidden" id="response_type" value="{response_type}">
            <input type="hidden" id="client_id" value="{client_id}">
            <input type="hidden" id="redirect_uri" value="{redirect_uri}">
            <input type="hidden" id="scope" value="{scope}">
            <input type="hidden" id="repsonse_type" value="{response_type}">
        </form>
    </div>

</body>
</html>
    "#
    );

    Ok(Response::builder()
        .status(200)
        .body(simple_login.to_string())
        .build())
}

pub fn auth_client_error() -> Result<Response> {
    let error_json = r#"{
    "error": "invalid_grant",
    "error_description": "The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client."
    }"#;

    Ok(Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(error_json)
        .build())
}

// POST /oauth/authorize
pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    let body = str::from_utf8(req.body()).unwrap();
    let a: HashMap<&str, &str> =
        querystring::querify(body).into_iter().collect();
    let username = a.get("Username").unwrap().to_string();
    let password = a.get("Password").unwrap().to_string();

    let referer = req.header("referer").unwrap().as_str().unwrap();
    let r = Url::parse(referer).unwrap();
    let hash_query: HashMap<_, _> = r.query_pairs().into_owned().collect();
    // {"client_id": "S8G2w1R95d5TDt5Psw80FNx5U4FWr2JHIV490VE61K8b", "redirect_uri": "icecubesapp://", "scope": "read write follow push", "response_type": "code"}
    tracing::debug!("referer ---> {referer}");
    tracing::debug!("hash_query ---> {hash_query:?}");

    let redirect_uri = hash_query.get("redirect_uri").unwrap();

    tracing::debug!("redirect_uri ---> {}", redirect_uri);

    // client_id should match with one in app_temp

    match check_password(username.clone(), password).await {
        true => {
            // Passed
            // update sparrow::mastodon::application

            // If password / auth is valid, the authorization code will be returned as a query parameter named code
            // This code is used to get a Token.
            // Code: String. A user authorization code, obtained via GET /oauth/authorize.
            // After this call 'POST /oauth/token HTTP/1.1' to get token
            // https://docs.joinmastodon.org/methods/oauth/#token

            let client_id = hash_query.get("client_id").unwrap().as_str();
            let application_json_string = String::from_utf8(
                sparrow::cache::get(client_id).await?.unwrap(),
            )
            .unwrap();

            // Generate Code
            let code = sparrow::utils::create_token().await;

            tracing::debug!("check_password_true");
            tracing::debug!(code);
            tracing::debug!(username);

            let user = sparrow::mastodon::account::Account::fr_username(
                Username(username),
            )
            .await?;
            let user_id = user.uid.to_string();

            let _ = sparrow::mastodon::application::Application::add(
                application_json_string,
                Some(user_id),
            )
            .await?;

            let body = format!(
                r#"<html><head>
                 <meta http-equiv="Refresh" content="0; URL={redirect_uri}redirect_uri?code={code}" />
                </head></html>"#
            );

            return Ok(Response::builder().status(200).body(body).build());
        }
        false => {
            // response_type, client_id and redirect_uri are mandatory fileds.
            // it there is missing,
            // return this: https://docs.joinmastodon.org/methods/oauth/#400-client-error
            let client_id = match hash_query.get("client_id") {
                Some(x) => x,
                None => {
                    return auth_client_error();
                }
            };
            let redirect_uri = match hash_query.get("redirect_uri") {
                Some(x) => x,
                None => {
                    return auth_client_error();
                }
            };
            let response_type = match hash_query.get("response_type") {
                Some(x) => x,
                None => {
                    return auth_client_error();
                }
            };
            let empty = &"".to_string();
            let scope = hash_query.get("scope").unwrap_or(empty);
            let redirect_location = format!("/oauth/authorize?auth_failed&client_id={client_id}&redirect_uri={redirect_uri}&response_type={response_type}&scope={scope}");

            return Ok(Response::builder()
                .status(303)
                .header("Location", redirect_location)
                .build());
        }
    }
}

async fn check_password(username: String, password: String) -> bool {
    tracing::debug!("Checking password");
    sparrow::mastodon::user::User::validate(username, password)
        .await
        .unwrap()
}
