use serde::Serialize;

#[derive(Serialize)]
pub struct Element {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub color: String
}