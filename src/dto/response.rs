#[derive(Debug, Clone)]
pub enum Response {
    Text(String),
    SessionSet,
    Json(String),
}
