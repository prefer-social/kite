#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Envelop<T> {
    pub address: String,
    pub letter: T,
}
