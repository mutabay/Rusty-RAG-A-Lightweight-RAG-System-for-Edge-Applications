use std::{fs::File, io::Write, path::Path};

pub fn save_file(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}
