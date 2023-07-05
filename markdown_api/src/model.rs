use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    pub content: String,
    pub created_at: String,
    pub title: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespDocument {
    pub id: Thing,
    pub content: String,
    pub created_at: String,
    pub title: String,
    pub updated_at: String,
}