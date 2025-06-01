use axum::{
    extract::Multipart,
    response::{IntoResponse},
    Json,
};
use std::path::PathBuf;


// [ Upload Document ] → [ Read Text ] → [ Embed Text (Ollama) ] → ✅ [ Store in FAISS ]

use crate::utils::file::{save_file, read_text_file};
use crate::utils::chunk::split_into_chunks;
use crate::services::embeddings::get_embedding;
use crate::services::semantics::send_to_faiss;

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {

        let file_name = field.file_name().unwrap_or("upload.dat").to_string();
        let data = field.bytes().await.unwrap();
        let path = PathBuf::from(format!("uploads/{}", file_name));
        if let Err(e) = save_file(&path, &data) {
            return Json(format!("❌ Failed to save file: {}", e)).into_response();
        }

        if let Ok(text) = read_text_file(&path) {
            let chunks = split_into_chunks(&text, 500); // 500-character chunks
        
            for (i, chunk) in chunks.iter().enumerate() {
                match get_embedding(&chunk).await {
                    Ok(vector) => {
                        let tag = format!("{} [chunk {}]", file_name, i + 1);
                        println!("🧠 Embedded chunk {} with {} chars", i + 1, chunk.len());
        
                        if let Err(e) = send_to_faiss(&vector, chunk).await {
                            println!("❌ Failed to store chunk in FAISS: {}", e);
                        } else {
                            println!("📤 Stored chunk: {}", tag);
                        }
                    }
                    Err(e) => println!("❌ Embedding failed on chunk {}: {}", i + 1, e),
                }
            }
        }
    }

    Json("✅ File uploaded and processed successfully").into_response()
}
