use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    #[serde(skip_deserializing)]  
    pub id: Uuid,
    pub title: String,
    pub content: String,
}
