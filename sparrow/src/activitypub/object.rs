// https://www.w3.org/TR/activitypub/#obj

pub struct Object {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    #[serde(rename = "type")]
    kind: String,
}
