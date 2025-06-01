use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub async fn get_embedding(text: &str) -> Result<Vec<f32>, reqwest::Error> {
    let url = env::var("OLLAMA_EMBEDDING_URL").expect("OLLAMA_EMBEDDING_URL not set");
    let model = env::var("OLLAMA_EMBEDDING_MODEL").expect("OLLAMA_EMBEDDING_MODEL not set");

    let payload = EmbeddingRequest {
        model: &model,
        prompt: text,
    };

    let client = Client::new();
    let res = client.post(url).json(&payload).send().await?;

    let parsed: EmbeddingResponse = res.json().await?;
    Ok(parsed.embedding)
}
