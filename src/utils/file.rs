use std::{fs::File, io::{Write, Read}, path::Path};


pub fn save_file(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn read_text_file(path: &Path) -> Result<String, std::io::Error> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext == "txt" || ext == "md" {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported file type"))
    }
}

