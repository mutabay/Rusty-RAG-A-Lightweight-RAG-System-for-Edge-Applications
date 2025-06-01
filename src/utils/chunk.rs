pub fn split_into_chunks(text: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for line in text.lines() {
        if current.len() + line.len() + 1 > max_chars {
            chunks.push(current.clone());
            current.clear();
        }

        current.push_str(line);
        current.push('\n');
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}
