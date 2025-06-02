use axum::{Json, response::IntoResponse};
use serde::Deserialize;

use crate::services::embeddings::get_embedding;
use crate::services::semantics::search_faiss;
use crate::services::llm::ask_llm;

#[derive(Deserialize)]
pub struct AskInput {
    pub question: String,
}

pub async fn ask(Json(payload): Json<AskInput>) -> impl IntoResponse {
    let question = payload.question;

    // 1. Embed question
    let Ok(vector) = get_embedding(&question).await else {
        return Json("❌ Failed to embed question").into_response();
    };

    // 2. Search FAISS
    let Ok(chunks) = search_faiss(&vector, 3).await else {
        return Json("❌ Failed to search FAISS").into_response();
    };

    // 3. Build prompt
    let prompt = format!(
        "Answer the question using the provided notes:\n---\n{}\n---\nQuestion: {}\nAnswer:",
        chunks.join("\n\n"),
        question
    );

    // 4. Call LLM
    match ask_llm(&prompt).await {
        Ok(answer) => Json(answer).into_response(),
        Err(e) => {
            eprintln!("❌ LLM error: {}", e);
            Json("❌ Failed to get LLM answer").into_response()
        }
    }
}
