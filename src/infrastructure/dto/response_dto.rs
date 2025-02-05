use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseBody {
    pub err: i32,
    #[serde(default)]
    pub data: Vec<Vec<serde_json::Value>>,
    pub content: String,
    pub duration: f64,
    pub re_code: String,
}

#[derive(Deserialize)]
pub struct PhoneticsResponse {
    pub(crate) us_ipa: String,
    pub(crate) uk_ipa: String,
}