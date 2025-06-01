use serde::Serialize;
use std::env;
use reqwest::Client;
use serde::Deserialize;

#[derive(Serialize)]
struct FaissAddRequest<'a> {
    embedding: &'a [f32],
    metadata: &'a str,
}

#[derive(Serialize)]
struct SearchRequest<'a> {
    query_vector: &'a [f32],
    k: usize,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<String>,
}


pub async fn send_to_faiss(embedding: &[f32], metadata: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let faiss_url = env::var("FAISS_ADD_URL").unwrap_or_else(|_| "http://localhost:8001/add".to_string());

    let payload = FaissAddRequest {
        embedding,
        metadata,
    };

    let res = client.post(&faiss_url).json(&payload).send().await?;

    if res.status().is_success() {
        println!("✅ Sent vector to FAISS");
    } else {
        println!("❌ FAISS error: {:?}", res.text().await?);
    }

    Ok(())
}

pub async fn search_faiss(query_vector: &[f32], k: usize) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let faiss_url = env::var("FAISS_SEARCH_URL").unwrap_or_else(|_| "http://localhost:8001/search".to_string());

    let payload = SearchRequest {
        query_vector,
        k,
    };

    let res = client.post(&faiss_url).json(&payload).send().await?;

    if res.status().is_success() {
        let parsed: SearchResponse = res.json().await?;
        Ok(parsed.results)
    } else {
        let text = res.text().await?;
        println!("❌ FAISS search error: {}", text);
        Ok(vec![])
    }
}
