use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}
