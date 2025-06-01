use axum::{Json, response::IntoResponse};
use serde::Deserialize;

use crate::services::embeddings::get_embedding;
use crate::services::semantics::search_faiss;

#[derive(Deserialize)]
pub struct QueryInput {
    pub question: String,
}

pub async fn query(Json(payload): Json<QueryInput>) -> impl IntoResponse {
    let query_text = &payload.question;

    match get_embedding(query_text).await {
        Ok(vector) => {
            match search_faiss(&vector, 3).await {
                Ok(results) => {
                    println!("🔍 Retrieved chunks: {:#?}", results);
                    Json(results).into_response()
                },
                Err(e) => {
                    eprintln!("❌ FAISS search error: {}", e);
                    Json("❌ Failed to search FAISS").into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Embedding error: {}", e);
            Json("❌ Failed to embed query").into_response()
        }
    }
}
