use axum::{
    extract::Multipart,
    response::{IntoResponse},
    Json,
};
use axum_extra::extract::multipart::Field;
use std::path::PathBuf;

use crate::utils::file::save_file;

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload.dat").to_string();
        let data = field.bytes().await.unwrap();

        let path = PathBuf::from(format!("uploads/{}", file_name));
        match save_file(&path, &data) {
            Ok(_) => continue,
            Err(e) => return Json(format!("❌ Failed to save file: {}", e)).into_response(),
        }
    }

    Json("✅ File uploaded successfully").into_response()
}
