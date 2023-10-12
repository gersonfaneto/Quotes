use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    pub book: String,
    pub quote: String,
}

#[derive(Serialize, FromRow)]
pub struct Quote {
    pub id: uuid::Uuid,
    pub book: String,
    pub quote: String,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    pub fn new(book: String, quote: String) -> Self {
        let current_time = chrono::Utc::now();

        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: current_time,
            updated_at: current_time,
        }
    }
}
