use anyhow;

use regex::Regex;

// @seungjin@mstd.seungjin.net ->
// seungjin@mstd.seungjin.net ->
// mstd.seungjin.net/@seungjin
// https://mstd.seungjin.net/users/seungjin ->
// https://mstd.seungjin.net/@seungjin ->

struct FederationId {
    id: String,
}

impl From for FederationId {
    
    fn from_string(arg: String) -> self {

        // starting with http, need to http request
        if start_with_http(arg) {

        }

    }
}

async fn start_with_http(a: String) -> bool {
    let re = Regex::new(r"^(http|https):\/\/").unwrap();
    re.is_match(a)
}

pub async fn validate_fed_id() -> Result<boolean> {
    Ok(true)
}

