use axum::{
    extract::Multipart,
    response::{IntoResponse},
    Json,
};
use std::path::PathBuf;

use crate::utils::file::{save_file, read_text_file};
use crate::services::embeddings::get_embedding;

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload.dat").to_string();
        let data = field.bytes().await.unwrap();

        let path = PathBuf::from(format!("uploads/{}", file_name));
        if let Err(e) = save_file(&path, &data) {
            return Json(format!("❌ Failed to save file: {}", e)).into_response();
        }
        
        match save_file(&path, &data) {
            Ok(_) => {
                if let Ok(text) = read_text_file(&path) {
                    match get_embedding(&text).await {
                        Ok(embedding) => {
                            println!("✅ Got embedding vector with {} dimensions", embedding.len());
                            println!("First 5 values: {:?}", &embedding[..5.min(embedding.len())]);
                        }
                        Err(err) => {
                            println!("❌ Embedding error: {}", err);
                            return Json("❌ Failed to embed content").into_response();
                        }
                    }
                }
            }
            Err(e) => return Json(format!("❌ Failed to save file: {}", e)).into_response(),
        }
    }

    Json("✅ File uploaded and processed successfully").into_response()
}
