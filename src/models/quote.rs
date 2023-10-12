use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

#[derive(Serialize)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
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
