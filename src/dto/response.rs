#[derive(Debug)]
pub enum Response {
    Text(String),
    SessionSet,
    Json(String),
}
