use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}
