use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestBody {
    pub(crate) re: bool,
    pub(crate) content: String,
}
